# Dora + Tauri

This is a dora + tauri project proof of concept. It's trying to show how to use dora with a GUI node build with Taur.


## How to build the project

```bash
cargo build
```

## How to run the project

Next inside the `gui-node` folder start the frontend server

```bash
cd ui-node
bun install
bun run dev
```

Finally, open a new terminal and run the following at the root of the project

```bash
dora start --name dora-tauri dataflow.yaml
```
