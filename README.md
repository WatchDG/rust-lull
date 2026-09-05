# rust-lull

Cargo workspace for lull: abstract engine contracts, a wire protocol, derive macros, and concrete engine implementations.

## Crates

| Crate | Role |
| --- | --- |
| [`lull-spec`](crates/lull-spec) | Abstract types and traits. No engine implementations. |
| [`lull-core`](crates/lull-core) | DAG graph, engine registry, and topological executor. |
| [`lull-protocol`](crates/lull-protocol) | Wire protocol messages and codec traits. |
| [`lull-derive`](crates/lull-derive) | Derive macros for `lull-spec` traits. |
| [`lull-engine`](crates/lull-engine) | Concrete engines that implement `lull-spec`. |

## License

Licensed under the [Apache License, Version 2.0](LICENSE).

Copyright 2026 Baranov Grigory Vladimirovich (WatchDG).
