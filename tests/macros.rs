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

fn result_to_bytes(res: hyper::Response) -> u8
{
    res.body().concat2().wait().unwrap().first().unwrap().clone()
}

/// Just to test the result of `mock_connector!` - this test was copied from hyper.
#[test]
fn test_redirect_followall() {
    let mut core = Core::new().unwrap();
    let client = hyper::client::Client::configure()
        .connector(MockRedirectPolicy::default())
        .build(&core.handle());
    //client.set_redirect_policy(hyper::client::RedirectPolicy::FollowAll);

    let req = client.get("http://127.0.0.1".parse().unwrap());
    let res = core.run(req).unwrap();
    assert_eq!(res.headers().get(), Some(&hyper::header::Server::new("mock3".to_owned())));
}

#[test]
fn test_sequential_mock() {
    let mut core = Core::new().unwrap();
    let client = hyper::client::Client::configure()
        .connector(MockSequential::default())
        .build(&core.handle());

    let req = client.get("http://127.0.0.1".parse().unwrap());
    let res = core.run(req).unwrap();
    assert_eq!(result_to_bytes(res), b'1');

    let req = client.get("http://127.0.0.1".parse().unwrap());
    let res = core.run(req).unwrap();
    assert_eq!(result_to_bytes(res), b'2');
}
