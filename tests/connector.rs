//! Test usage of connectors which work entirely without macros
extern crate hyper;
extern crate yup_hyper_mock;
extern crate tokio_core;
extern crate futures;

use tokio_core::reactor::Core;
use futures::future::Future;
 use futures::Stream;

use yup_hyper_mock::{SequentialConnector, HostToReplyConnector};

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

impl hyper::client::Service for MySequentialConnector {
    type Request = hyper::Uri;
    type Response = tokio_core::net::TcpStream;
    type Error = std::io::Error;
    type Future = yup_hyper_mock::HttpConnecting;

    fn call(&self, _uri: Self::Request) -> Self::Future {
        unimplemented!();
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

impl hyper::client::Service for MyHostToReplyConnector {
    type Request = hyper::Uri;
    type Response = tokio_core::net::TcpStream;
    type Error = std::io::Error;
    type Future = yup_hyper_mock::HttpConnecting;

    fn call(&self, _uri: Self::Request) -> Self::Future {
        unimplemented!();
    }
}

fn result_to_bytes(res: hyper::Response) -> u8
{
    res.body().concat2().wait().unwrap().first().unwrap().clone()
}

#[test]
fn test_sequential_mock() {
    let core = Core::new().unwrap();
    let client = hyper::client::Client::configure()
        .connector(MySequentialConnector::default())
        .build(&core.handle());

    let res = client.get("http://127.0.0.1".parse().unwrap()).wait().unwrap();
    assert_eq!(result_to_bytes(res), b'1');

    let res = client.get("http://127.0.0.1".parse().unwrap()).wait().unwrap();
    assert_eq!(result_to_bytes(res), b'2');
}


/// Just to test the result of `mock_connector!` - this test was copied from hyper.
#[test]
fn test_redirect_followall() {
    let core = Core::new().unwrap();
    let client = hyper::client::Client::configure()
        .connector(MyHostToReplyConnector::default())
        .build(&core.handle());
    //client.set_redirect_policy(hyper::client::RedirectPolicy::FollowAll);

    let res = client.get("http://127.0.0.1".parse().unwrap()).wait().unwrap();
    assert_eq!(res.headers().get(), Some(&hyper::header::Server::new("mock3".to_owned())));
}