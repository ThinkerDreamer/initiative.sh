#!/bin/bash
set -euxo pipefail

git ls-files '*.rs' | xargs rustfmt --check
cargo clippy --workspace -- --deny warnings
cargo test --workspace
(cd web && wasm-pack test --firefox --headless)
