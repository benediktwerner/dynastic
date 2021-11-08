# Dynastic

Genealogy software written in Rust and Typescript/Svelte.

## Dev Setup

### Requirements

- Rust
- Node

### Generating Typescript types

When changing `src/model.rs`: Run `./gen-ts.sh` to regenerate `svelte/src/data.d.ts`

### Compile and run frontend (svelte)

1. `cd svelte`
2. `npm install`
3. Dev server: `npm run dev`
4. Compile prod version: `npm run build`

### Compile and run server

- Dev: `npm run tauri dev`
- Build prod version: `npm run tauri build`
