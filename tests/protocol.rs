use backupd::Handshake;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::net::{TcpListener, TcpStream};

#[test]
fn test_slow_handshake() {
    let listen = TcpListener::bind("127.0.0.1:34215").expect("failed to create listener");
    let t = thread::spawn(move || {
        let mut stream = TcpStream::connect("127.0.0.1:34215").unwrap();
        let handshake = Handshake{ version: 1, name: "name".into(), secret: "secret".into() };
        let data = bincode::serialize(&handshake).unwrap();
        stream.write_all(&data[..5]).unwrap();
        thread::sleep(Duration::from_secs(1));

        stream.write_all(&data[5..]).unwrap();
    });

    let (stream, _) = listen.accept().unwrap();
    let handshake: Handshake = bincode::deserialize_from(stream).expect("Failed to read handshake");
    assert_eq!(handshake.version, 1);
    assert_eq!(handshake.name, "name");
    assert_eq!(handshake.secret, "secret");

    t.join().unwrap();
}
