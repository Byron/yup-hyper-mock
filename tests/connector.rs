//! Test usage of connectors which work entirely without macros
extern crate hyper;
extern crate yup_hyper_mock;

use yup_hyper_mock::{SequentialConnector, HostToReplyConnector, MockStream};

struct MySequentialConnector(SequentialConnector);

impl Default for MySequentialConnector {
    fn default() -> MySequentialConnector {
        let mut c = MySequentialConnector(Default::default());
        c.0.content.push("HTTP/1.1 200 OK\r\n\
                         Server: BOGUS\r\n\
                         \r\n\
                         1".to_string());

        c.0.content.push("HTTP/1.1 200 OK\r\n\
                         Server: BOGUS\r\n\
                         \r\n\
                         2".to_string());
        c
    }
}

impl hyper::net::NetworkConnector for MySequentialConnector {
    type Stream = MockStream;

    fn connect(&self, host: &str, port: u16, scheme: &str) -> ::hyper::Result<MockStream> {
        self.0.connect(host, port, scheme)
    }
}

struct MyHostToReplyConnector(HostToReplyConnector);

impl Default for MyHostToReplyConnector {
    fn default() -> MyHostToReplyConnector {
        let mut c = MyHostToReplyConnector(Default::default());

        c.0.m.insert("http://127.0.0.1".to_string(),
                     "HTTP/1.1 301 Redirect\r\n\
                     Location: http://127.0.0.2\r\n\
                     Server: mock1\r\n\
                     \r\n\
                    ".to_string());
        c.0.m.insert("http://127.0.0.2".to_string(),
                     "HTTP/1.1 302 Found\r\n\
                     Location: https://127.0.0.3\r\n\
                     Server: mock2\r\n\
                     \r\n\
                    ".to_string());
        c.0.m.insert("https://127.0.0.3".to_string(),
                      "HTTP/1.1 200 OK\r\n\
                      Server: mock3\r\n\
                      \r\n\
                      ".to_string());
        c
    }
}

impl hyper::net::NetworkConnector for MyHostToReplyConnector {
    type Stream = MockStream;

    fn connect(&self, host: &str, port: u16, scheme: &str) -> ::hyper::Result<MockStream> {
        self.0.connect(host, port, scheme)
    }
}

#[test]
fn test_sequential_mock() {
    use std::io::Read;

    let client = hyper::client::Client::with_connector(MySequentialConnector::default());

    let res = client.get("http://127.0.0.1").send().unwrap();
    assert_eq!(res.bytes().next().unwrap().unwrap(), b'1');

    let res = client.get("http://127.0.0.1").send().unwrap();
    assert_eq!(res.bytes().next().unwrap().unwrap(), b'2');
}


/// Just to test the result of `mock_connector!` - this test was copied from hyper.
#[test]
fn test_redirect_followall() {
    let mut client = hyper::client::Client::with_connector(MyHostToReplyConnector::default());
    client.set_redirect_policy(hyper::client::RedirectPolicy::FollowAll);

    let res = client.get("http://127.0.0.1").send().unwrap();
    assert_eq!(res.headers.get(), Some(&hyper::header::Server("mock3".to_owned())));
}