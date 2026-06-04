# SPDX-License-Identifier: MPL-2.0
# Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
import Config

config :logger, level: :info

config :coordinator,
  work_generation_interval: 10_000,
  redundancy: 3,
  quorum: 2
