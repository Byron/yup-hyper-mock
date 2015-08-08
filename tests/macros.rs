extern crate hyper;
#[macro_use]
extern crate yup_hyper_mock;
#[macro_use]
extern crate log;

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


/// Just to test the result of `mock_connector!` - this test was copied from hyper.
#[test]
fn test_redirect_followall() {
    let mut client = hyper::client::Client::with_connector(MockRedirectPolicy::default());
    client.set_redirect_policy(hyper::client::RedirectPolicy::FollowAll);

    let res = client.get("http://127.0.0.1").send().unwrap();
    assert_eq!(res.headers.get(), Some(&hyper::header::Server("mock3".to_owned())));
}

#[test]
fn test_sequential_mock() {
    use std::io::Read;

    let client = hyper::client::Client::with_connector(MockSequential::default());

    let res = client.get("http://127.0.0.1").send().unwrap();
    assert_eq!(res.bytes().next().unwrap().unwrap(), b'1');

    let res = client.get("http://127.0.0.1").send().unwrap();
    assert_eq!(res.bytes().next().unwrap().unwrap(), b'2');
}
