#!/bin/sh

set -euxo pipefail

cd "$( dirname "${BASH_SOURCE[0]}" )"

(cd src-tauri; cargo run --bin gen-ts --features="schemars" -- ../schemas)

node gen-ts.js schemas src/data.d.ts
