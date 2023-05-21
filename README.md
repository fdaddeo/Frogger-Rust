# Frogger Rust

This repo contains a personal Rust implementation for the Frogger game, realized for the course of Functional Programming.

## Build Instructions

### Requirement

To build the game are required:
- `pyhton3`
- `wasm-pack`. Refer to this [link](https://rustwasm.github.io/wasm-pack/installer/).

### Build

Open a terminal window and type:

```
wasm-pack build --target web
```

## Play Instructions

To play the game open a terminal window and type:

```
python3 -m serve
```

Then open your browser and search for `localhost:8000`.
You have to play with the arrows. Have fun!

## Mentions
Some code was borrowed from our professor's [repo](https://github.com/tomamic/bounce-rust).