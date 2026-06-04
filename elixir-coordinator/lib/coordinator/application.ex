# SPDX-License-Identifier: MPL-2.0
# Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
defmodule Coordinator.Application do
  @moduledoc """
  OTP Application for Oblibeny BOINC Coordinator.

  Supervises all components for distributed verification coordination.
  """

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      # Database connection pool
      {Coordinator.Database, []},

      # Work generation supervisor
      {Task.Supervisor, name: Coordinator.WorkSupervisor},

      # Core GenServers
      {Coordinator.WorkGenerator, []},
      {Coordinator.ResultValidator, []},
      {Coordinator.ProofTracker, []},
      {Coordinator.PropertyManager, []},

      # Telemetry
      Coordinator.Telemetry
    ]

    opts = [strategy: :one_for_one, name: Coordinator.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
