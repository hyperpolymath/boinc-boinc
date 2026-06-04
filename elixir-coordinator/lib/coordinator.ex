# SPDX-License-Identifier: MPL-2.0
# Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
defmodule Coordinator do
  @moduledoc """
  Oblibeny BOINC Coordinator - Main API module.
  """

  alias Coordinator.{WorkGenerator, ResultValidator, ProofTracker, PropertyManager}

  @doc """
  Generate work units for a specific property.
  """
  defdelegate generate_work(property_id, count), to: WorkGenerator, as: :generate_batch

  @doc """
  Submit a result for validation.
  """
  defdelegate submit_result(work_unit_id, volunteer_id, result),
    to: ResultValidator

  @doc """
  Get proof status for a property.
  """
  defdelegate proof_status(property_id), to: ProofTracker

  @doc """
  Get overall verification progress.
  """
  defdelegate overall_progress(), to: ProofTracker

  @doc """
  Get all properties being verified.
  """
  defdelegate all_properties(), to: PropertyManager

  @doc """
  Get work generation statistics.
  """
  defdelegate work_stats(), to: WorkGenerator, as: :stats

  @doc """
  Get result validation statistics.
  """
  defdelegate validation_stats(), to: ResultValidator, as: :stats
end
