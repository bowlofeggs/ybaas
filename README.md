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
