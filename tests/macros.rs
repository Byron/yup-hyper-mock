extern crate hyper;
#[macro_use]
extern crate yup_hyper_mock;
//#[macro_use]
extern crate log;
extern crate futures;
extern crate tokio_core;

use futures::future::Future;
use futures::Stream;
use tokio_core::reactor::Core;
use hyper::{Body, header, client::Client, Response};

mock_connector_in_order! (MockSequential {
                                  "HTTP/1.1 200 OK\r\n\
                                 Server: BOGUS\r\n\
                                 \r\n\
                                 1"

                                 "HTTP/1.1 200 OK\r\n\
                                 Server: BOGUS\r\n\
                                 \r\n\
                                 2"
                                  });


mock_connector!(MockRedirectPolicy {
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

fn result_to_bytes(res: Response<Body>) -> String {
    let mut body = res.into_body().wait();
    let items = body.get_mut().concat2().wait().unwrap().to_vec();
    String::from_utf8(items).unwrap()
}

/// Just to test the result of `mock_connector!` - this test was copied from hyper.
#[test]
fn test_redirect_followall() {
    let client = Client::builder()
        .build::<MockRedirectPolicy, Body>(MockRedirectPolicy::default());

    fn check_server(client: &Client<MockRedirectPolicy, Body>, url: &str, server: &str) {
        let req = client.get(url.parse().unwrap());
        let mut core = Core::new().unwrap();
        let res = core.run(req).unwrap();
        let header = header::HeaderValue::from_str(&server).unwrap();
        assert_eq!(res.headers().get(header::SERVER), Some(&header));
    }
    check_server(&client, "http://127.0.0.1", "mock1");
    check_server(&client, "http://127.0.0.2", "mock2");
    check_server(&client, "https://127.0.0.3", "mock3");
}

#[test]
fn test_sequential_mock() {
    let mut core = Core::new().unwrap();
    let client = Client::builder()
        .build::<MockSequential, Body>(MockSequential::default());

    let req = client.get("http://127.0.0.1".parse().unwrap());
    let res = core.run(req).unwrap();
    assert_eq!(result_to_bytes(res), "1");

    let req = client.get("http://127.0.0.1".parse().unwrap());
    let res = core.run(req).unwrap();
    assert_eq!(result_to_bytes(res), "2");
}
