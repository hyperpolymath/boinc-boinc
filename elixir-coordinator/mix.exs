# SPDX-License-Identifier: MPL-2.0
# Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
defmodule Coordinator.MixProject do
  use Mix.Project

  def project do
    [
      app: :coordinator,
      version: "0.6.0",
      elixir: "~> 1.15",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger],
      mod: {Coordinator.Application, []}
    ]
  end

  defp deps do
    [
      {:arangox, "~> 0.5"},
      {:jason, "~> 1.4"},
      {:poolboy, "~> 1.5"},
      {:telemetry, "~> 1.2"},
      {:telemetry_metrics, "~> 0.6"},
      {:telemetry_poller, "~> 1.0"},
      {:httpoison, "~> 2.2"},
      {:uuid, "~> 1.1"},
      {:nimble_options, "~> 1.1"}
    ]
  end
end
