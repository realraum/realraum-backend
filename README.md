# Realraum Backend collection

A monorepo for the various backends of [Realraum](https://realraum.at/).

- [Sounds Backend](/sounds/README.md), a sound-playing server backend
- [Projector Backend](/projector/README.md), a projector remote-control server backend

The frontend can be found in [realraum-frontend](https://github.com/realraum/realraum-frontend),
which is a separate repository.
It's written in [Leptos](https://leptos.dev/), a Solid-inspired web framework for Rust,
though it may be rewritten in [Solid.js](https://www.solidjs.com/) in the future.

Use

```zsh
cross build --release --target=arm-unknown-linux-gnueabihf
```

to build for the Raspberry Pi using [cross](https://github.com/cross-rs/cross).

## Configuration

Provide something like `0.0.0.0:4242` to the env vars

- `R3_SOUNDS_ADDR`, and
- `R3_PROJECTOR_ADDR`

to configure the ip addresses and ports of the sounds and the projector backends, respectively.
