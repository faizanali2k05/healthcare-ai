# rust-crates workspace

Five v0.1 crates targeting researched gaps in the Rust ecosystem (2025–2026):

| Crate | Gap it fills | Deps |
|---|---|---|
| `anystash` | Modern typemap (unmaintained `typemap` has no clean successor) | none |
| `segfen` | Generic monoid segment tree + Fenwick tree, no_std | none |
| `voxelops` | Scientific voxel processing (documented in not-yet-awesome-rust) | none |
| `ogc-gml` | OGC GML 3.2 <-> geo-types (documented in not-yet-awesome-rust) | quick-xml, geo-types |
| `sentencize` | Sentence segmentation for RAG/NLP pipelines | none |

Build & test everything: `cargo test --workspace`

Before publishing any crate: verify the name with `cargo publish --dry-run`, add your repository URL in `Cargo.toml`, and add LICENSE-MIT / LICENSE-APACHE files.
