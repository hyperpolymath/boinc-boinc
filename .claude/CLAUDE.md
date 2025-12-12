# Project Instructions

## ⚠️ CONVERSION REQUIRED: TypeScript/JavaScript → ReScript

This repository needs FULL conversion from TS/JS to ReScript.

### Policy (RSR)
- **REQUIRED**: ReScript for all code
- **FORBIDDEN**: TypeScript (.ts, .tsx)
- **FORBIDDEN**: JavaScript (.js, .jsx) except generated .res.js
- **EXCEPTION**: WASM for performance-critical computation

### When modifying this codebase:
1. DO NOT add new TS/JS files
2. When touching existing TS/JS, convert to ReScript
3. Create `.res` file with equivalent logic
4. Delete the original TS/JS file
5. Run `rescript build` to generate JS

### DO NOT:
- Add new TypeScript files
- Write raw JavaScript
- Use tsc or TypeScript compiler

See TS_CONVERSION_NEEDED.md for full migration guide.

## Exception: deployment/arangodb/init-db.js

This file is an ArangoDB Foxx initialization script. It runs inside ArangoDB's
embedded JavaScript engine, not in browser or Node.js. Converting to ReScript
is not applicable for this database deployment artifact.

## Container Policy (RSR)

### Primary Stack
- **Runtime**: nerdctl (not docker)
- **Base Image**: wolfi (cgr.dev/chainguard/wolfi-base)
- **Distroless**: Use distroless variants where possible

### Fallback Stack
- **Runtime**: podman (if nerdctl unavailable)
- **Base Image**: alpine (if wolfi unavailable)

### DO NOT:
- Use `docker` command (use `nerdctl` or `podman`)
- Use Dockerfile (use Containerfile)
- Use debian/ubuntu base images (use wolfi/alpine)
