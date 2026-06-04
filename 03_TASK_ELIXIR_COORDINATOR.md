<!--
SPDX-License-Identifier: MPL-2.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# Task 2: Elixir/OTP Coordinator

## Objective
Build distributed coordination system for managing BOINC work units, result aggregation, and proof orchestration using Elixir/OTP.

## Deliverables

### 1. OTP Application Structure
- Supervision tree for fault tolerance
- GenServers for state management
- Task supervisors for work generation
- Pooling for database connections

### 2. Work Generator
- Generate test programs from grammar
- Create work units with redundancy
- Batch work unit creation
- Priority queue for properties

### 3. Result Validator
- Aggregate redundant results
- Quorum consensus (2/3)
- Detect Byzantine failures
- Update proof progress

### 4. ArangoDB Integration
- Connection pooling
- AQL query builders
- Transaction support
- Graph traversal for proof dependencies

### 5. BOINC Integration
- Work unit XML generation
- Result processing
- Volunteer tracking
- Credit/badging system

## Implementation

### Supervision Tree
```elixir
defmodule Coordinator.Application do
  use Application

  def start(_type, _args) do
    children = [
      {Coordinator.Database, []},
      {Coordinator.WorkGenerator, []},
      {Coordinator.ResultValidator, []},
      {Coordinator.ProofTracker, []},
      {Task.Supervisor, name: Coordinator.TaskSupervisor}
    ]

    Supervisor.start_link(children, strategy: :one_for_one)
  end
end
```

### Work Generator
```elixir
defmodule Coordinator.WorkGenerator do
  use GenServer

  def start_link(_) do
    GenServer.start_link(__MODULE__, %{}, name: __MODULE__)
  end

  def generate_batch(property_id, count) do
    GenServer.call(__MODULE__, {:generate, property_id, count})
  end

  def handle_call({:generate, property_id, count}, _from, state) do
    work_units = Enum.map(1..count, fn _ ->
      program = generate_test_program(property_id)
      create_work_unit(program, property_id)
    end)

    {:reply, work_units, state}
  end
end
```

### Database Module
```elixir
defmodule Coordinator.Database do
  use GenServer

  def query(aql, vars \\ %{}) do
    GenServer.call(__MODULE__, {:query, aql, vars})
  end

  def insert_work_unit(unit) do
    aql = """
    INSERT @unit INTO work_units
    RETURN NEW
    """
    query(aql, %{unit: unit})
  end
end
```

## File Structure
```
elixir-coordinator/
├── mix.exs
├── config/
│   ├── config.exs
│   ├── dev.exs
│   └── prod.exs
├── lib/
│   ├── coordinator.ex
│   ├── coordinator/
│   │   ├── application.ex
│   │   ├── work_generator.ex
│   │   ├── result_validator.ex
│   │   ├── proof_tracker.ex
│   │   ├── database.ex
│   │   └── boinc_integration.ex
│   └── coordinator_web/
│       └── api/
└── test/
```

## Dependencies
```elixir
defp deps do
  [
    {:arangox, "~> 0.5"},
    {:jason, "~> 1.4"},
    {:poolboy, "~> 1.5"},
    {:telemetry, "~> 1.2"},
    {:telemetry_metrics, "~> 0.6"}
  ]
end
```
