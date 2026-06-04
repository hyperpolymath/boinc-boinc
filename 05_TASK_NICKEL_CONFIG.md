<!--
SPDX-License-Identifier: MPL-2.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# Task 4: Nickel Configuration System

## Objective
Type-safe configuration management for all deployment environments.

## Deliverables

### 1. Schema Definition
```nickel
{
  ServerConfig = {
    host | String | default = "0.0.0.0",
    port | Number | default = 4000,

    database | {
      url | String,
      name | String | default = "oblibeny_boinc",
      username | String,
      password | String | doc "Database password (use secrets)",
      pool_size | Number | default = 10,
    },

    boinc | {
      project_name | String | default = "Oblibeny Verification",
      project_url | String,
      redundancy | Number | default = 3,
      quorum | Number | default = 2,
      work_unit_timeout | Number | default = 300,
    },

    resources | {
      max_work_units | Number | default = 10000,
      generation_rate | Number | default = 100,
    },
  },

  validate_config = fun config =>
    if config.boinc.quorum > config.boinc.redundancy then
      error "Quorum cannot exceed redundancy"
    else
      config,
}
```

### 2. Environment Configs
- Development
- Staging
- Production

### 3. Secrets Management
- Environment variable injection
- Secret rotation support
