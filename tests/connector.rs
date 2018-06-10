//! Test usage of connectors which work entirely without macros
extern crate hyper;
extern crate yup_hyper_mock;
extern crate tokio_core;
extern crate env_logger;

use tokio_core::reactor::Core;
use hyper::{Body, client::Client, header, rt::Future, rt::Stream, Response};

use yup_hyper_mock::{SequentialConnector, HostToReplyConnector};

fn result_to_bytes(res: Response<Body>) -> String {
    let mut body = res.into_body().wait();
    let items = body.get_mut().concat2().wait().unwrap().to_vec();
    String::from_utf8(items).unwrap()
}

#[test]
fn test_sequential_mock() {
    env_logger::try_init().ok();
    let mut c = SequentialConnector::default();
    c.content.push("HTTP/1.1 200 OK\r\n\
                        Server: BOGUS\r\n\
                        \r\n\
                        1".to_string());

    c.content.push("HTTP/1.1 200 OK\r\n\
                        Server: BOGUS\r\n\
                        \r\n\
                        2".to_string());

    let mut core = Core::new().unwrap();
    let client = Client::builder()
        .build::<SequentialConnector, Body>(c);

    let req = client.get("http://127.0.0.1".parse().unwrap());
    let res = core.run(req).unwrap();
    assert_eq!(result_to_bytes(res), "1");

    let req = client.get("http://127.0.0.1".parse().unwrap());
    let res = core.run(req).unwrap();
    assert_eq!(result_to_bytes(res), "2");
}

/// Just to test the result of `mock_connector!` - this test was copied from hyper.
#[test]
fn test_redirect_followall() {
    env_logger::try_init().ok();
    let mut c = HostToReplyConnector::default();
    c.m.insert("http://127.0.0.1".to_string(),
                    "HTTP/1.1 301 Redirect\r\n\
                    Location: http://127.0.0.2\r\n\
                    Server: mock1\r\n\
                    \r\n\
                ".to_string());
    c.m.insert("http://127.0.0.2".to_string(),
                    "HTTP/1.1 302 Found\r\n\
                    Location: https://127.0.0.3\r\n\
                    Server: mock2\r\n\
                    \r\n\
                ".to_string());
    c.m.insert("http://127.0.0.3".to_string(),
                    "HTTP/1.1 200 OK\r\n\
                    Server: mock3\r\n\
                    \r\n\
                    ".to_string());

    let client = Client::builder()
        .build::<HostToReplyConnector, Body>(c);

    fn check_server(client: &Client<HostToReplyConnector, Body>, url: &str, server: &str) {
        let req = client.get(url.parse().unwrap());
        let mut core = Core::new().unwrap();
        let res = core.run(req).unwrap();
        let header = header::HeaderValue::from_str(&server).unwrap();
        assert_eq!(res.headers().get(header::SERVER), Some(&header));
    }
    check_server(&client, "http://127.0.0.1", "mock1");
    check_server(&client, "http://127.0.0.2", "mock2");
    check_server(&client, "http://127.0.0.3", "mock3");
}