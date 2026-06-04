# SPDX-License-Identifier: MPL-2.0
# Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
defmodule Coordinator.WorkGenerator do
  @moduledoc """
  Generates BOINC work units for Oblibeny verification.

  Responsibilities:
  - Generate test programs from grammar
  - Create work units with appropriate redundancy
  - Prioritize properties needing verification
  - Track work unit distribution
  """

  use GenServer
  require Logger

  alias Coordinator.Database

  @redundancy 3
  @generation_interval 5_000 # 5 seconds

  defmodule State do
    defstruct [
      :generation_timer,
      :work_queue,
      :stats,
      :properties
    ]
  end

  def start_link(opts) do
    GenServer.start_link(__MODULE__, opts, name: __MODULE__)
  end

  @impl true
  def init(_opts) do
    # Load properties to verify
    {:ok, properties} = load_properties()

    state = %State{
      generation_timer: schedule_generation(),
      work_queue: :queue.new(),
      stats: %{generated: 0, distributed: 0},
      properties: properties
    }

    Logger.info("WorkGenerator started")

    {:ok, state}
  end

  ## Public API

  @doc """
  Generate a batch of work units for a specific property.
  """
  def generate_batch(property_id, count) do
    GenServer.call(__MODULE__, {:generate_batch, property_id, count})
  end

  @doc """
  Get current generation statistics.
  """
  def stats do
    GenServer.call(__MODULE__, :stats)
  end

  ## Callbacks

  @impl true
  def handle_call({:generate_batch, property_id, count}, _from, state) do
    work_units =
      Enum.map(1..count, fn _i ->
        generate_work_unit(property_id)
      end)

    # Store work units in database
    Enum.each(work_units, fn unit ->
      Database.insert("work_units", unit)
    end)

    new_stats = %{state.stats | generated: state.stats.generated + count}

    {:reply, {:ok, work_units}, %{state | stats: new_stats}}
  end

  @impl true
  def handle_call(:stats, _from, state) do
    {:reply, state.stats, state}
  end

  @impl true
  def handle_info(:generate_work, state) do
    # Periodic work generation
    # Prioritize properties with lowest coverage

    property = select_priority_property(state.properties)

    case generate_work_unit(property["_key"]) do
      {:ok, unit} ->
        Database.insert("work_units", unit)
        new_stats = %{state.stats | generated: state.stats.generated + 1}
        {:noreply, %{state | stats: new_stats}}

      {:error, reason} ->
        Logger.error("Failed to generate work unit: #{inspect(reason)}")
        {:noreply, state}
    end

    # Schedule next generation
    timer = schedule_generation()
    {:noreply, %{state | generation_timer: timer}}
  end

  ## Private Functions

  defp schedule_generation do
    Process.send_after(self(), :generate_work, @generation_interval)
  end

  defp load_properties do
    case Database.all("properties") do
      {:ok, properties} -> {:ok, properties}
      {:error, _} -> {:ok, default_properties()}
    end
  end

  defp default_properties do
    [
      %{
        "_key" => "1",
        "name" => "Phase Separation Soundness",
        "priority" => 10
      },
      %{
        "_key" => "2",
        "name" => "Deployment Termination",
        "priority" => 10
      },
      %{
        "_key" => "3",
        "name" => "Resource Bounds Enforcement",
        "priority" => 9
      },
      %{
        "_key" => "4",
        "name" => "Capability System Soundness",
        "priority" => 9
      },
      %{
        "_key" => "5",
        "name" => "Obfuscation Semantic Preservation",
        "priority" => 8
      },
      %{
        "_key" => "6",
        "name" => "Call Graph Acyclicity",
        "priority" => 10
      },
      %{
        "_key" => "7",
        "name" => "Memory Safety",
        "priority" => 10
      }
    ]
  end

  defp select_priority_property(properties) do
    # Simple priority-based selection
    # In production, would consider coverage, recent failures, etc.
    Enum.max_by(properties, & &1["priority"], fn -> List.first(properties) end)
  end

  defp generate_work_unit(property_id) do
    # Generate a random Oblibeny program that tests the property

    program = generate_test_program(property_id)

    unit = %{
      "_key" => UUID.uuid4(),
      "property_id" => property_id,
      "program" => program,
      "status" => "pending",
      "redundancy" => @redundancy,
      "results" => [],
      "created_at" => DateTime.utc_now() |> DateTime.to_iso8601(),
      "timeout_seconds" => 300,
      "memory_limit_mb" => 512
    }

    {:ok, unit}
  end

  defp generate_test_program(property_id) do
    # Program generation based on property being tested

    case property_id do
      "1" -> generate_phase_separation_test()
      "2" -> generate_termination_test()
      "3" -> generate_resource_bounds_test()
      "4" -> generate_capability_test()
      "5" -> generate_obfuscation_test()
      "6" -> generate_call_graph_test()
      "7" -> generate_memory_safety_test()
      _ -> generate_random_program()
    end
  end

  defp generate_phase_separation_test do
    # Generate program that should pass phase separation checks
    """
    (defun-deploy test-phase-separation (n) : int32
      (let ((sum 0))
        (bounded-for i 0 n
          (set sum (+ sum i)))
        sum))
    """
  end

  defp generate_termination_test do
    # Generate program with bounded loops only
    loop_count = Enum.random(1..20)

    """
    (defun-deploy test-termination (x) : int32
      (bounded-for i 0 #{loop_count}
        (+ x i))
      x)
    """
  end

  defp generate_resource_bounds_test do
    # Generate program with measurable resource usage
    iterations = Enum.random(10..100)

    """
    (program test-resources
      (resource-budget
        (time-ms 10000)
        (memory-bytes 1024))

      (defun-deploy compute () : int32
        (let ((result 0))
          (bounded-for i 0 #{iterations}
            (set result (+ result (* i i))))
          result)))
    """
  end

  defp generate_capability_test do
    # Test capability system
    """
    (defun-deploy test-capability (gpio-cap) : void
      (with-capability gpio-cap
        (gpio-set gpio-cap 1)
        (sleep-ms 100)
        (gpio-set gpio-cap 0)))
    """
  end

  defp generate_obfuscation_test do
    # Generate program to test semantic preservation
    """
    (defun-deploy fibonacci (n) : int32
      (let ((a 0) (b 1) (temp 0))
        (bounded-for i 0 n
          (set temp (+ a b))
          (set a b)
          (set b temp))
        a))
    """
  end

  defp generate_call_graph_test do
    # Test non-recursive call patterns
    """
    (defun-deploy helper (x) : int32
      (+ x 1))

    (defun-deploy main (n) : int32
      (helper (helper n)))
    """
  end

  defp generate_memory_safety_test do
    # Test array bounds checking
    size = Enum.random(10..100)

    """
    (defun-deploy test-array-safety () : int32
      (let ((arr (array int32 #{size})))
        (bounded-for i 0 #{size}
          (array-set arr i (* i 2)))
        (array-get arr #{size - 1})))
    """
  end

  defp generate_random_program do
    # Fallback: simple random program
    """
    (defun-deploy random-test (n) : int32
      (bounded-for i 0 10
        (+ n i))
      n)
    """
  end
end
