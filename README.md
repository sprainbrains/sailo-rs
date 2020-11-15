# Sailo-rs

The library for Sailors that want to write SailfishOS apps in Rust.
The goal of this project is to make writing Rust applications for SailfishOS as
simple as they are with Python and C++, if not simpler, and provide a framework
with the least amount of overhead possible.

## Build images

This crate contains Docker images, based on the Platform SDK
(Dockerized [by CODeRUS](https://github.com/CODeRUS/docker-sailfishos-platform-sdk/),
thank you!), that contain three Rust compilers (stable, beta, nightly).

Use them as `registry.gitlab.com/rubdos/sailo-rs/platform-$MER_ARCH-$SFOS_VERSION:master`, with the following parameters:

### `MER_ARCH`

- `armv7hl`
- `aarch64`
- `i486`

### SFOS_VERSION

- `3.4.0.24`
