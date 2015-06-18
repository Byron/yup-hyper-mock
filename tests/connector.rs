//! Test usage of connectors which work entirely without macros
extern crate hyper;
extern crate yup_hyper_mock;

use yup_hyper_mock::{SequentialConnector, MockStream};

struct MySequentialConnector(SequentialConnector);

impl Default for MySequentialConnector {
    fn default() -> MySequentialConnector {
        let c = MySequentialConnector(Default::default());
        c.0.content.borrow_mut().push("HTTP/1.1 200 OK\r\n\
                                 Server: BOGUS\r\n\
                                 \r\n\
                                 1".to_string());

        c.0.content.borrow_mut().push("HTTP/1.1 200 OK\r\n\
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

    fn set_ssl_verifier(&mut self, _: hyper::net::ContextVerifier) {}
}

#[test]
fn test_sequential_mock() {
    use std::io::Read;

    let mut client = hyper::client::Client::with_connector(MySequentialConnector::default());

    let res = client.get("http://127.0.0.1").send().unwrap();
    assert_eq!(res.bytes().next().unwrap().unwrap(), b'1');

    let res = client.get("http://127.0.0.1").send().unwrap();
    assert_eq!(res.bytes().next().unwrap().unwrap(), b'2');
}
