<!--
SPDX-License-Identifier: CC-BY-SA-4.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# Task 6: Phoenix Dashboard

## Objective
Real-time web dashboard for monitoring Oblibeny BOINC verification progress.

## Deliverables

### 1. Phoenix Application
- LiveView for real-time updates
- REST API for external access
- WebSocket connections
- Authentication

### 2. Features

#### Overview Page
- Total work units (pending/completed)
- Active volunteers
- Verification progress by property
- Recent activity feed

#### Property Details
- Progress percentage
- Test coverage
- Counterexamples (if any)
- Proof status

#### Volunteer Management
- Volunteer list with reliability scores
- Work distribution
- Performance metrics

#### Graph Visualizations
- Proof dependency graph (from ArangoDB)
- Program variant relationships
- Property coverage

### 3. Components
```elixir
defmodule DashboardWeb.OverviewLive do
  use DashboardWeb, :live_view

  def mount(_params, _session, socket) do
    if connected?(socket) do
      :timer.send_interval(1000, self(), :update)
    end

    {:ok, assign(socket, load_stats())}
  end

  def handle_info(:update, socket) do
    {:noreply, assign(socket, load_stats())}
  end
end
```

### 4. API Endpoints
- `GET /api/stats` - Overall statistics
- `GET /api/properties/:id` - Property details
- `GET /api/work_units` - Work unit list
- `GET /api/volunteers` - Volunteer list
- `GET /api/proofs/:id` - Proof status

## File Structure
```
phoenix-dashboard/
├── mix.exs
├── assets/
│   ├── css/
│   ├── js/
│   └── vendor/
├── lib/
│   ├── dashboard.ex
│   └── dashboard_web/
│       ├── live/
│       ├── controllers/
│       ├── views/
│       └── templates/
└── test/
```
