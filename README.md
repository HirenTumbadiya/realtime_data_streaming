Realtime Data Streaming
=======================

Purpose
-------
This repository is a Rust crate that implements a modular real-time data streaming stack. It centers on processing and transporting time-series events (ticks and sequences) via pluggable components:

- `clock` — time utilities and scheduling
- `source` — ingestion/source adapters
- `transport` — network or in-memory transport implementations
- `protocol` — message formats and (de)serialization for ticks/sequences
- `engine` — core processing, routing, and buffering
- `consumer` / `handler` — downstream consumers and event handlers

What we'll build
----------------
- A minimal end-to-end example wiring a `source` to an `engine` and a `consumer`.
- A simple transport (in-memory or TCP) and a lightweight protocol for tick/sequence messages.
- Tests, examples, and basic CI to validate builds and behavior.

Build & Run
-----------
Requirements: Rust toolchain (stable, installed via `rustup`).

To build:

```powershell
cargo build
```

To run (if an executable example is provided later):

```powershell
cargo run --example <example_name>
```

Files
-----
- Project plan: [project_plan.txt](project_plan.txt)

Next steps
----------
1. Review `project_plan.txt` for milestones.
2. Add a minimal example under `examples/` and pick an async runtime (e.g., `tokio`) if needed for network transports.

