// FIXME
#![allow(dead_code, unused_imports, unreachable_code)]

//! Test usage of connectors which work entirely without macros
extern crate hyper;
extern crate yup_hyper_mock;
extern crate tokio_core;
extern crate futures;
extern crate http;
extern crate env_logger;

use tokio_core::reactor::Core;
use futures::Future;
use futures::Stream;

use yup_hyper_mock::{SequentialConnector, HostToReplyConnector};

struct MyHostToReplyConnector(HostToReplyConnector);

impl Default for MyHostToReplyConnector {
    fn default() -> MyHostToReplyConnector {
        let mut c = MyHostToReplyConnector(Default::default());

        c.0.m.insert("http://127.0.0.1".to_string(),
                     "HTTP/1.1 301 Redirect\r\n\
                     Location: http://127.0.0.2\r\n\
                     Server: mock1\r\n\
                     Content-Length: 0
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
    type Response = std::io::Cursor<Vec<u8>>;
    type Error = std::io::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, uri: Self::Request) -> Self::Future {
        self.0.call(uri)
    }
}

fn result_to_bytes(res: hyper::Response) -> String {
    let mut body = res.body().wait();
    let items = body.get_mut().concat2().wait().unwrap().to_vec();
    String::from_utf8(items).unwrap()
}

#[test]
fn test_sequential_mock() {
    let mut c = SequentialConnector::default();
    c.content.push("HTTP/1.1 200 OK\r\n\
                        Server: BOGUS\r\n\
                        \r\n\
                        1".to_string());

    c.content.push("HTTP/1.1 200 OK\r\n\
                        Server: BOGUS\r\n\
                        \r\n\
                        2".to_string());

    env_logger::init();
    let mut core = Core::new().unwrap();
    let client = hyper::client::Client::configure()
        .connector(c)
        .keep_alive(false)
        .build(&core.handle());

    let req = client.get("http://127.0.0.1".parse().unwrap());
    let res = core.run(req).unwrap();
    assert_eq!(result_to_bytes(res), '1'.to_string());

    // let req = client.get("http://127.0.0.1".parse().unwrap());
    // let res = core.run(req).unwrap();
    // assert_eq!(result_to_bytes(res), '2'.to_string());
}

/// Just to test the result of `mock_connector!` - this test was copied from hyper.
#[test]
fn test_redirect_followall() {
    unimplemented!();
    let mut core = Core::new().unwrap();
    let client = hyper::client::Client::configure()
        .connector(MyHostToReplyConnector::default())
        .build(&core.handle());
    //client.set_redirect_policy(hyper::client::RedirectPolicy::FollowAll);

    let req = client.get("http://127.0.0.1".parse().unwrap());
    let res = core.run(req).unwrap();
    assert_eq!(res.headers().get(), Some(&hyper::header::Server::new("mock3".to_owned())));
}