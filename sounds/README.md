# Realraum Sounds Backend

[![github]](https://github.com/realraum/realraum-backend)&ensp;[![crates-io]](https://crates.io/crates/realraum_backend_sounds)&ensp;[![docs-rs]](https://docs.rs/realraum_backend_sounds/latest)

A sound-playing server backend for Realraum; work in progress.

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
  - Rust is a modern systems programming language focusing on safety, speed, and concurrency
  - It empowers everyone to build reliable and efficient software
  - Realraum Sounds Backend is written in Rust, compiled to ARMv7, and runs on a Raspberry Pi
- Cross compilation to ARMv7
  - Get something like `gcc-arm-linux-gnueabihf` from your package manager
  - `rustup target add armv7-unknown-linux-gnueabihf`

## License

[![GNU Affero General Public License v3.0](https://www.gnu.org/graphics/agplv3-with-text-162x68.png)](https://www.gnu.org/licenses/agpl-3.0.html)

Copyright (C) 2023 [@Tanja-4732](https://github.com/Tanja-4732)

Realraum Sounds Backend is free software: you can redistribute it and/or modify it under the terms of the [GNU Affero General Public License](/LICENSE.md) as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

Realraum Sounds Backend is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the [GNU Affero General Public License](/LICENSE.md) for more details.

You should have received a copy of the [GNU Affero General Public License](/LICENSE.md) along with Realraum Sounds Backend. If not, see <https://www.gnu.org/licenses/>, specifically <https://www.gnu.org/licenses/agpl-3.0.html>.

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
[wiki-nim]: https://en.wikipedia.org/wiki/Nim
