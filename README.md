```ybaas``` (Yubibomb as a Service) is a Rust microservice that gives you a hosted
[yubibomb](https://crates.io/crates/yubibomb) so your users don't have to deal with the
complexities of installing and using the command line tool itself.
You can use it to pretend to accidentally press your Yubikey while your IRC
client has the keyboard focus! Usage example:

```
$ curl http://localhost:3030/hotp
869926
```

This project is available on [crates.io](https://crates.io/crates/ybaas).

# Install

## DockerHub

`ybaas` is available on DockerHub for `x86_64` systems. You can run it with
[`podman`](https://podman.io/) or [`docker`](https://www.docker.com/products/container-runtime). For
example:

```
$ podman run -d --net=host docker.io/bowlofeggs/ybaas:latest
$ curl http://localhost:3030/hotp
375633
```


## GitHub

Linux `x86_64` executables are available from GitHub at
https://github.com/bowlofeggs/ybaas/releases.


## Build from source

You can [install Rust](https://www.rust-lang.org/tools/install), and then build it from source
yourself:

```
$ git clone git@github.com:bowlofeggs/ybaas.git
$ cd ybaas
$ cargo build --release
$ ./target/release/ybaas &
$ curl http://localhost:3030/hotp
171033
```
