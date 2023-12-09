[![Test status](https://img.shields.io/github/actions/workflow/status/sytten/rusty-ghost/test.yaml?style=for-the-badge)](https://github.com/Sytten/rusty-ghost/actions/workflows/test.yaml)
[![Docker version](https://img.shields.io/docker/v/sytten/rusty-ghost?style=for-the-badge&label=Docker&color=blue)](https://hub.docker.com/r/sytten/rusty-ghost)
![License](https://img.shields.io/badge/LICENSE-MIT-green?style=for-the-badge)

<p align="center">
  <img src="https://github.com/Sytten/rusty-ghost/assets/2366731/7124cadd-2f00-4b9d-9b5a-2657b2fabbdf" width="100"/>

  <h3 align="center">Rusty Ghost</h3>

  <p align="center">
    A Bittorrent Proxy for tracker reports
  </p>
</p>

# About

## Usage

With Docker:
`docker run -p 3773:3773 sytten/rusty-ghost`

With cargo:
`cargo run -- --port 3773 --zero-dl --private-key ./ca/rusty-ghost.key --ca-cert ./ca/rusty-ghost.cer`

## Generate CA

We provide a general certificate authority, but you can generate your own if you prefer.

1. In the `ca` folder run `openssl re
2. You can hit `Enter` to accept most default options, but fill at least a `Common Name`
3. You have to override the certificate in docker
   - By rebuilding it `docker build -t rusty-ghost:latest`
   - By mounting a volume on top on existing keys `docker run -v /path/to/cert.cer:/ca/rusty-ghost.cer -v /path/to/private.key:/ca/rusty-ghost.key sytten/rusty-ghost`

## Inspirations

- [Ratio Ghost](https://github.com/ratioghost/ratioghost)
- [cheatproxy](https://github.com/drguildo/cheatproxy)
- [Torrent Ratio Proxy](https://github.com/warren-bank/node-torrent-client-ratio-modifier-http-proxy)
