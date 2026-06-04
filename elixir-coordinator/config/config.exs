# SPDX-License-Identifier: MPL-2.0
# Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
import Config

config :coordinator,
  arangodb: [
    endpoints: System.get_env("ARANGO_URL") || "http://localhost:8529",
    database: System.get_env("ARANGO_DB") || "oblibeny_boinc",
    username: System.get_env("ARANGO_USER") || "root",
    password: System.get_env("ARANGO_PASSWORD") || ""
  ]

# Import environment specific config
import_config "#{config_env()}.exs"
