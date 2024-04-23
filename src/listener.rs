use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::sync::Mutex;
use std::sync::Arc;


pub fn main() -> std::io::Result<()> {
  let count: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
  let listener = TcpListener::bind("127.0.0.1:7910")?;

  // accept connections and process them serially
  for stream in listener.incoming() {
    handle_client(stream?, count.clone());
  }
  Ok(())
}

fn handle_client(mut stream: TcpStream, count: Arc<Mutex<u64>>) {
  std::thread::spawn(move || {
      let mut buf = [0; 512];
      stream.read(&mut buf).unwrap();
      let mut count = count.lock().unwrap();
      *count += 1;
      println!("count: {}", *count);
  });
}
