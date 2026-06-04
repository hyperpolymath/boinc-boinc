# SPDX-License-Identifier: MPL-2.0
# Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
defmodule Coordinator.ResultValidator do
  @moduledoc """
  Validates results from BOINC volunteers using quorum consensus.

  Implements Byzantine fault tolerance through redundancy and voting.
  """

  use GenServer
  require Logger

  alias Coordinator.Database

  @quorum_threshold 2 # 2 out of 3 must agree

  defmodule State do
    defstruct [:pending_validations, :stats]
  end

  def start_link(opts) do
    GenServer.start_link(__MODULE__, opts, name: __MODULE__)
  end

  @impl true
  def init(_opts) do
    state = %State{
      pending_validations: %{},
      stats: %{validated: 0, invalid: 0, pending: 0}
    }

    Logger.info("ResultValidator started")

    {:ok, state}
  end

  ## Public API

  @doc """
  Submit a result for validation.
  """
  def submit_result(work_unit_id, volunteer_id, result) do
    GenServer.cast(__MODULE__, {:submit_result, work_unit_id, volunteer_id, result})
  end

  @doc """
  Get validation statistics.
  """
  def stats do
    GenServer.call(__MODULE__, :stats)
  end

  ## Callbacks

  @impl true
  def handle_cast({:submit_result, work_unit_id, volunteer_id, result}, state) do
    # Store result in database
    result_doc = %{
      "_key" => UUID.uuid4(),
      "work_unit_id" => work_unit_id,
      "volunteer_id" => volunteer_id,
      "result" => result,
      "submitted_at" => DateTime.utc_now() |> DateTime.to_iso8601(),
      "validation_status" => "pending"
    }

    Database.insert("results", result_doc)

    # Check if we have enough results for validation
    case get_work_unit_results(work_unit_id) do
      {:ok, results} when length(results) >= @quorum_threshold + 1 ->
        # We have enough results, perform validation
        validate_results(work_unit_id, results)
        {:noreply, state}

      {:ok, _results} ->
        # Not enough results yet
        {:noreply, state}

      {:error, reason} ->
        Logger.error("Failed to get results: #{inspect(reason)}")
        {:noreply, state}
    end
  end

  @impl true
  def handle_call(:stats, _from, state) do
    {:reply, state.stats, state}
  end

  ## Private Functions

  defp get_work_unit_results(work_unit_id) do
    aql = """
    FOR r IN results
      FILTER r.work_unit_id == @work_unit_id
      RETURN r
    """

    Database.query(aql, %{work_unit_id: work_unit_id})
  end

  defp validate_results(work_unit_id, results) do
    # Quorum-based validation
    # Group results by hash of output
    grouped =
      results
      |> Enum.group_by(fn r -> hash_result(r["result"]) end)

    # Find consensus
    case find_consensus(grouped) do
      {:ok, consensus_hash, consensus_results} ->
        # Mark consensus results as valid
        Enum.each(consensus_results, fn r ->
          Database.update("results", r["_key"], %{"validation_status" => "valid"})
          update_volunteer_stats(r["volunteer_id"], :valid)
        end)

        # Mark non-consensus results as invalid
        non_consensus =
          results
          |> Enum.reject(fn r -> hash_result(r["result"]) == consensus_hash end)

        Enum.each(non_consensus, fn r ->
          Database.update("results", r["_key"], %{"validation_status" => "invalid"})
          update_volunteer_stats(r["volunteer_id"], :invalid)
        end)

        # Update work unit status
        Database.update("work_units", work_unit_id, %{
          "status" => "validated",
          "consensus_result" => List.first(consensus_results)["result"]
        })

        Logger.info("Work unit #{work_unit_id} validated with quorum #{length(consensus_results)}")

      {:error, :no_consensus} ->
        # No quorum reached, request more results
        Logger.warning("No consensus for work unit #{work_unit_id}, requesting more results")

        Database.update("work_units", work_unit_id, %{
          "redundancy" => 5 # Increase redundancy
        })
    end
  end

  defp find_consensus(grouped) do
    # Find group with >= quorum_threshold results
    case Enum.find(grouped, fn {_hash, results} ->
           length(results) >= @quorum_threshold
         end) do
      {hash, results} -> {:ok, hash, results}
      nil -> {:error, :no_consensus}
    end
  end

  defp hash_result(result) do
    # Simple hash for result comparison
    # In production, would use proper cryptographic hash
    :erlang.phash2(result)
  end

  defp update_volunteer_stats(volunteer_id, status) do
    case Database.get("volunteers", volunteer_id) do
      {:ok, volunteer} ->
        updates =
          case status do
            :valid -> %{"valid_results" => (volunteer["valid_results"] || 0) + 1}
            :invalid -> %{"invalid_results" => (volunteer["invalid_results"] || 0) + 1}
          end

        # Update reliability score
        total = (volunteer["valid_results"] || 0) + (volunteer["invalid_results"] || 0) + 1
        valid = updates["valid_results"] || volunteer["valid_results"] || 0
        score = valid / total

        updates = Map.put(updates, "reliability_score", score)

        Database.update("volunteers", volunteer_id, updates)

      {:error, :not_found} ->
        # Create volunteer record
        initial_stats =
          case status do
            :valid -> %{"valid_results" => 1, "invalid_results" => 0}
            :invalid -> %{"valid_results" => 0, "invalid_results" => 1}
          end

        volunteer = Map.merge(initial_stats, %{
          "_key" => volunteer_id,
          "joined_at" => DateTime.utc_now() |> DateTime.to_iso8601(),
          "reliability_score" => if(status == :valid, do: 1.0, else: 0.0)
        })

        Database.insert("volunteers", volunteer)
    end
  end
end
