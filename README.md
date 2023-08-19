# mini-redis-rpc

[![Crates.io][crates-badge]][crates-url]
[![License][license-badge]][license-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/mini-redis-rpc.svg
[crates-url]: https://crates.io/crates/mini-redis-rpc
[license-badge]: https://img.shields.io/crates/l/mini-redis-rpc.svg
[license-url]: #license
[actions-badge]: https://github.com/Millione/mini-redis-rpc/actions/workflows/ci.yaml/badge.svg
[actions-url]: https://github.com/Millione/mini-redis-rpc/actions

A [Redis](https://redis.io) client and server built with [Volo](https://github.com/Cloudwego/Volo).

The set of commands Redis provides can be found
[here](https://redis.io/commands).

## Running

Start the server:

```
cargo run --bin mini-redis-rpc-server
```

A CLI client is provided to run arbitrary commands from the
terminal. With the server running, the following works in a
different terminal window:

```
cargo run --bin mini-redis-rpc-cli set foo bar

cargo run --bin mini-redis-rpc-cli get foo
```

## Acknowledgements
[mini-reids](https://github.com/tokio-rs/mini-redis)

## License

Dual-licensed under the MIT license and the Apache License (Version 2.0).

See [LICENSE-MIT](https://github.com/Millione/mini-redis-rpc/blob/main/LICENSE-MIT) and [LICENSE-APACHE](https://github.com/Millione/mini-redis-rpc/blob/main/LICENSE-APACHE) for details.
