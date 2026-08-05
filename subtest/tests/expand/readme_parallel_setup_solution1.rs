use subtest::subtest;

#[subtest]
#[test]
fn server_starts() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap(); // <-- OS-assigned port
    let port = listener.local_addr().unwrap().port();
    assert_ne!(port, 0);

    #[subtest]
    fn server_accepts_a_connection() {
        std::net::TcpStream::connect(("127.0.0.1", port)).unwrap();
    }
}
