# SPDX-License-Identifier: MPL-2.0
# Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
defmodule Coordinator.Telemetry do
  @moduledoc """
  Telemetry setup for monitoring coordinator performance.
  """

  use Supervisor
  require Logger

  def start_link(opts) do
    Supervisor.start_link(__MODULE__, opts, name: __MODULE__)
  end

  @impl true
  def init(_opts) do
    children = [
      # Telemetry poller for periodic measurements
      {:telemetry_poller, measurements: periodic_measurements(), period: 10_000}
    ]

    Supervisor.init(children, strategy: :one_for_one)
  end

  defp periodic_measurements do
    [
      # VM measurements
      {__MODULE__, :measure_memory, []},
      {__MODULE__, :measure_process_count, []}
    ]
  end

  def measure_memory do
    :erlang.memory()
    |> Enum.each(fn {key, value} ->
      :telemetry.execute([:vm, :memory], %{value: value}, %{kind: key})
    end)
  end

  def measure_process_count do
    count = length(:erlang.processes())
    :telemetry.execute([:vm, :process_count], %{count: count}, %{})
  end
end
