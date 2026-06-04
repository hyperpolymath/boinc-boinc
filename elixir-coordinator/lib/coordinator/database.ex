# SPDX-License-Identifier: MPL-2.0
# Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
defmodule Coordinator.Database do
  @moduledoc """
  ArangoDB connection manager with connection pooling.
  """

  use GenServer
  require Logger

  @pool_size 10
  @pool_overflow 5

  def start_link(opts) do
    GenServer.start_link(__MODULE__, opts, name: __MODULE__)
  end

  @impl true
  def init(_opts) do
    config = Application.get_env(:coordinator, :arangodb, [])

    {:ok, conn} =
      Arangox.start_link(
        endpoints: config[:endpoints] || "http://localhost:8529",
        database: config[:database] || "oblibeny_boinc",
        username: config[:username] || "root",
        password: config[:password] || "",
        pool_size: @pool_size
      )

    Logger.info("ArangoDB connection established")

    {:ok, %{conn: conn}}
  end

  ## Public API

  @doc """
  Execute an AQL query.
  """
  def query(aql, vars \\ %{}) do
    GenServer.call(__MODULE__, {:query, aql, vars})
  end

  @doc """
  Insert a document into a collection.
  """
  def insert(collection, document) do
    aql = """
    INSERT @doc INTO #{collection}
    RETURN NEW
    """

    query(aql, %{doc: document})
  end

  @doc """
  Update a document.
  """
  def update(collection, key, updates) do
    aql = """
    UPDATE @key WITH @updates IN #{collection}
    RETURN NEW
    """

    query(aql, %{key: key, updates: updates})
  end

  @doc """
  Get a document by key.
  """
  def get(collection, key) do
    aql = """
    RETURN DOCUMENT(#{collection}, @key)
    """

    case query(aql, %{key: key}) do
      {:ok, [doc]} -> {:ok, doc}
      {:ok, []} -> {:error, :not_found}
      error -> error
    end
  end

  @doc """
  Get all documents from a collection.
  """
  def all(collection, opts \\ []) do
    limit = Keyword.get(opts, :limit, 1000)
    offset = Keyword.get(opts, :offset, 0)

    aql = """
    FOR doc IN #{collection}
      LIMIT @offset, @limit
      RETURN doc
    """

    query(aql, %{offset: offset, limit: limit})
  end

  @doc """
  Count documents in a collection.
  """
  def count(collection) do
    aql = "RETURN COUNT(#{collection})"

    case query(aql, %{}) do
      {:ok, [count]} -> {:ok, count}
      error -> error
    end
  end

  @doc """
  Graph traversal for proof dependencies.
  """
  def traverse_proof_dependencies(proof_id, depth \\ 10) do
    aql = """
    FOR v, e, p IN 1..@depth OUTBOUND @start_vertex
      GRAPH 'proof_dependencies'
      RETURN p
    """

    query(aql, %{start_vertex: "proofs/#{proof_id}", depth: depth})
  end

  ## Callbacks

  @impl true
  def handle_call({:query, aql, vars}, _from, %{conn: conn} = state) do
    result =
      case Arangox.post(conn, "/_api/cursor", %{query: aql, bindVars: vars}) do
        {:ok, %{body: %{"result" => results}}} ->
          {:ok, results}

        {:error, %{status: status, body: body}} ->
          Logger.error("ArangoDB query failed: #{status} - #{inspect(body)}")
          {:error, {:query_failed, status, body}}

        error ->
          Logger.error("ArangoDB query error: #{inspect(error)}")
          {:error, :query_error}
      end

    {:reply, result, state}
  end
end
