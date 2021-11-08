#!/bin/sh

set -euxo pipefail

cd "$( dirname "${BASH_SOURCE[0]}" )"

(cd src-tauri; cargo run --bin gen-ts --features="schemars" -- ../schema.json)

node gen-ts.js schema.json src/data.d.ts
