<!--
SPDX-License-Identifier: MPL-2.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# Task 3: Nix Build System

## Objective
Create reproducible builds for all Oblibeny BOINC components using Nix flakes.

## Deliverables

### 1. Main Flake
```nix
{
  description = "Oblibeny BOINC Platform";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in {
        packages = {
          rust-parser = pkgs.callPackage ./rust-parser {};
          elixir-coordinator = pkgs.callPackage ./elixir-coordinator {};
          lean-proofs = pkgs.callPackage ./lean-proofs {};
          phoenix-dashboard = pkgs.callPackage ./phoenix-dashboard {};
          all = pkgs.symlinkJoin {
            name = "oblibeny-all";
            paths = with self.packages.${system}; [
              rust-parser elixir-coordinator lean-proofs phoenix-dashboard
            ];
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" "rust-analyzer" ];
            })
            elixir
            lean4
            nodejs
            arangodb
            podman
          ];
        };
      }
    );
}
```

### 2. Component Packages
- Rust parser Nix derivation
- Elixir coordinator package
- Lean proofs build
- Phoenix dashboard assets

### 3. Cross-compilation
- Linux x86_64
- Linux ARM64 (for volunteers)
- macOS support

### 4. Binary Cache
- Setup Cachix or self-hosted cache
- Pre-build common dependencies
- Fast CI/CD builds
