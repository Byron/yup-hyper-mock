[![Rust](https://github.com/Byron/yup-hyper-mock/workflows/Rust/badge.svg)](https://github.com/Byron/yup-hyper-mock/actions)
[![Crates.io](https://img.shields.io/crates/v/yup-hyper-mock.svg)](https://crates.io/crates/yup-hyper-mock)

`hyper-mock` is a utility library to help hyper clients with their testing. It provides types used to test hyper itself, most notably, mock connections and macros to ease their use.

## Usage

Set it up for use in tests in `Cargo.toml`
```toml
[dev-dependencies]
yup-hyper-mock = "*"
log = "*"  # log macros are used within yup-hyper-mock
```

In your tests module
```Rust
#[cfg(test)]
mod tests {
    use hyper;
    use hyper_util::client::legacy::Client;

    yup_hyper_mock::mock_connector!(MockRedirectPolicy {
        "http://127.0.0.1" =>       "HTTP/1.1 301 Redirect\r\n\
                                     Location: http://127.0.0.2\r\n\
                                     Server: mock1\r\n\
                                     \r\n\
                                    "
        "http://127.0.0.2" =>       "HTTP/1.1 302 Found\r\n\
                                     Location: https://127.0.0.3\r\n\
                                     Server: mock2\r\n\
                                     \r\n\
                                    "
        "https://127.0.0.3" =>      "HTTP/1.1 200 OK\r\n\
                                     Server: mock3\r\n\
                                     \r\n\
                                    "
    });

    #[tokio::test]
    async fn test_redirect_followall() {
        let builder =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new());
        let client: Client<MockRedirectPolicy, http_body_util::Empty<hyper::body::Bytes>> =
            builder.build(MockRedirectPolicy::default());

        let res = client
            .get(hyper::Uri::from_static("http://127.0.0.1"))
            .await
            .unwrap();

        let headers = res.headers();
        assert!(headers.contains_key("Server"));
        assert_eq!(headers["Server"], "mock1");
    }
}
```

## Credits

`yup-hyper-mock` is code from `hyper/src/mock.rs`, which was adjusted to work within its very own crate.


## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
