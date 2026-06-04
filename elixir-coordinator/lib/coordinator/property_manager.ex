# SPDX-License-Identifier: MPL-2.0
# Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
defmodule Coordinator.PropertyManager do
  @moduledoc """
  Manages the 7 properties being verified for Oblibeny.
  """

  use GenServer
  require Logger

  alias Coordinator.Database

  @properties [
    %{
      id: "1",
      name: "Phase Separation Soundness",
      description: "No compile-time construct appears in deploy-time code",
      priority: 10,
      formal_statement: "∀ program P, ∀ expression E ∈ deploy-phase(P), compile-only-construct(E) → False"
    },
    %{
      id: "2",
      name: "Deployment Termination",
      description: "All deploy-time code provably terminates",
      priority: 10,
      formal_statement: "∀ program P, ∀ input I, ∃ n ∈ ℕ, eval(deploy-phase(P), I, n) = value"
    },
    %{
      id: "3",
      name: "Resource Bounds Enforcement",
      description: "Resource usage never exceeds declared budgets",
      priority: 9,
      formal_statement: "∀ program P with budget B, ∀ execution trace T, resources(T) ≤ B"
    },
    %{
      id: "4",
      name: "Capability System Soundness",
      description: "I/O operations only succeed within capability scope and budget",
      priority: 9,
      formal_statement: "∀ I/O operation Op, ∀ execution E, success(Op, E) → ∃ capability C ∈ scope(E), allows(C, Op)"
    },
    %{
      id: "5",
      name: "Obfuscation Semantic Preservation",
      description: "Code morphing preserves program semantics",
      priority: 8,
      formal_statement: "∀ program P, ∀ morphed variant P', ∀ input I, eval(P, I) ≈ eval(P', I)"
    },
    %{
      id: "6",
      name: "Call Graph Acyclicity",
      description: "No recursion in deploy-time code",
      priority: 10,
      formal_statement: "∀ program P, call-graph(deploy-phase(P)) is acyclic"
    },
    %{
      id: "7",
      name: "Memory Safety",
      description: "All memory accesses within bounds",
      priority: 10,
      formal_statement: "∀ array access A[i] in execution E, 0 ≤ i < length(A)"
    }
  ]

  def start_link(opts) do
    GenServer.start_link(__MODULE__, opts, name: __MODULE__)
  end

  @impl true
  def init(_opts) do
    # Initialize properties in database
    Enum.each(@properties, fn prop ->
      Database.insert("properties", prop)
    end)

    Logger.info("PropertyManager initialized with #{length(@properties)} properties")

    {:ok, %{properties: @properties}}
  end

  ## Public API

  @doc """
  Get all properties.
  """
  def all_properties do
    @properties
  end

  @doc """
  Get property by ID.
  """
  def get_property(id) do
    Enum.find(@properties, fn p -> p.id == id end)
  end

  @doc """
  Get properties sorted by priority.
  """
  def prioritized_properties do
    Enum.sort_by(@properties, & &1.priority, :desc)
  end
end
