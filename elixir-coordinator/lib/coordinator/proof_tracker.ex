# SPDX-License-Identifier: MPL-2.0
# Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
defmodule Coordinator.ProofTracker do
  @moduledoc """
  Tracks formal proof progress and manages proof dependencies.
  """

  use GenServer
  require Logger

  alias Coordinator.Database

  defmodule State do
    defstruct [:proofs, :coverage, :last_update]
  end

  def start_link(opts) do
    GenServer.start_link(__MODULE__, opts, name: __MODULE__)
  end

  @impl true
  def init(_opts) do
    state = %State{
      proofs: load_proofs(),
      coverage: %{},
      last_update: DateTime.utc_now()
    }

    Logger.info("ProofTracker started")

    {:ok, state}
  end

  ## Public API

  @doc """
  Update proof progress based on validated results.
  """
  def update_proof_coverage(property_id, program_id, result) do
    GenServer.cast(__MODULE__, {:update_coverage, property_id, program_id, result})
  end

  @doc """
  Get proof status for a property.
  """
  def proof_status(property_id) do
    GenServer.call(__MODULE__, {:proof_status, property_id})
  end

  @doc """
  Get overall proof progress.
  """
  def overall_progress do
    GenServer.call(__MODULE__, :overall_progress)
  end

  ## Callbacks

  @impl true
  def handle_cast({:update_coverage, property_id, program_id, result}, state) do
    # Update coverage tracking
    key = {property_id, program_id}

    new_coverage =
      Map.update(state.coverage, key, [result], fn existing ->
        [result | existing]
      end)

    # Update database
    coverage_doc = %{
      "property_id" => property_id,
      "program_id" => program_id,
      "result" => result,
      "timestamp" => DateTime.utc_now() |> DateTime.to_iso8601()
    }

    Database.insert("coverage", coverage_doc)

    # Check if proof progress has been made
    check_proof_progress(property_id, new_coverage)

    {:noreply, %{state | coverage: new_coverage, last_update: DateTime.utc_now()}}
  end

  @impl true
  def handle_call({:proof_status, property_id}, _from, state) do
    status = get_property_status(property_id, state.coverage)
    {:reply, status, state}
  end

  @impl true
  def handle_call(:overall_progress, _from, state) do
    progress = calculate_overall_progress(state)
    {:reply, progress, state}
  end

  ## Private Functions

  defp load_proofs do
    case Database.all("proofs") do
      {:ok, proofs} -> proofs
      {:error, _} -> []
    end
  end

  defp get_property_status(property_id, coverage) do
    # Count test cases for this property
    test_count =
      coverage
      |> Enum.filter(fn {{prop_id, _}, _} -> prop_id == property_id end)
      |> length()

    # Check for counterexamples
    counterexamples =
      coverage
      |> Enum.filter(fn {{prop_id, _}, results} ->
        prop_id == property_id and Enum.any?(results, & &1["counterexample"])
      end)
      |> length()

    %{
      property_id: property_id,
      test_cases: test_count,
      counterexamples: counterexamples,
      status:
        cond do
          counterexamples > 0 -> :counterexample_found
          test_count > 1_000_000 -> :strong_evidence
          test_count > 100_000 -> :moderate_evidence
          test_count > 10_000 -> :weak_evidence
          true -> :insufficient_data
        end
    }
  end

  defp calculate_overall_progress(state) do
    # Calculate progress across all 7 properties
    properties = 1..7

    property_progress =
      Enum.map(properties, fn id ->
        status = get_property_status(to_string(id), state.coverage)
        {id, status}
      end)
      |> Map.new()

    total_tests =
      state.coverage
      |> Enum.map(fn {_, results} -> length(results) end)
      |> Enum.sum()

    %{
      properties: property_progress,
      total_test_cases: total_tests,
      last_update: state.last_update,
      overall_confidence: calculate_confidence(property_progress)
    }
  end

  defp calculate_confidence(property_progress) do
    # Simple confidence calculation based on test counts
    scores =
      Enum.map(property_progress, fn {_id, status} ->
        case status.status do
          :counterexample_found -> 0.0
          :strong_evidence -> 0.95
          :moderate_evidence -> 0.80
          :weak_evidence -> 0.60
          :insufficient_data -> 0.20
        end
      end)

    Enum.sum(scores) / length(scores)
  end

  defp check_proof_progress(property_id, coverage) do
    status = get_property_status(property_id, coverage)

    case status.status do
      :counterexample_found ->
        Logger.warning("Counterexample found for property #{property_id}!")
        # Notify proof system

      :strong_evidence ->
        Logger.info("Strong evidence accumulated for property #{property_id}")
        # Could trigger formal proof attempt

      _ ->
        :ok
    end
  end
end
