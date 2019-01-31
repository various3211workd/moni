use std::io::*;
use std::net::{TcpStream, SocketAddr, Shutdown};

pub fn read() -> String {
  
  let addrs = [
    SocketAddr::from(([127, 0, 0, 1], 12749)),
  ];
  
  let mut stream = TcpStream::connect(&addrs[..])
    .expect("Couldn't connect to the server...");

  let mut buf = vec![];

  loop {  // ここで止まる
    match stream.read_to_end(&mut buf) {
      Ok(_) => break,
      Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
      }
      Err(e) => panic!("encountered IO error: {}", e),
    };
  };

  let line: String = 
    String::from_utf8(buf.to_vec()).unwrap();

  stream.shutdown(Shutdown::Both)
    .expect("shutdown call failed");

  line
}

pub fn send(line: String) {
  let addrs = [
    SocketAddr::from(([127, 0, 0, 1], 12749)),
  ];

  let mut stream = TcpStream::connect(&addrs[..])
    .expect("Couldn't connect to the server...");

  let bytes: &[u8] = line.as_bytes();
  loop {
    match stream.write(bytes) {
      Ok(_) => break,
      Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
      }
      Err(e) => panic!("encountered IO error: {}", e),
    };
  };

  stream.shutdown(Shutdown::Both)
    .expect("shutdown call failed");
}