use serial_test::serial;
use subtest::subtest;

#[subtest]
#[test]
#[serial] // <-- inherited by the subtest, so the two never run at the same time
fn server_starts() {
    let listener = std::net::TcpListener::bind("127.0.0.1:39118").unwrap(); // <-- still a fixed port
    let port = listener.local_addr().unwrap().port();
    assert_eq!(port, 39118);

    #[subtest]
    fn server_accepts_a_connection() {
        std::net::TcpStream::connect(("127.0.0.1", port)).unwrap();
    }
}
