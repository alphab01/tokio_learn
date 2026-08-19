use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
  let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

  loop {
    let (socket, _) = listener.accept().await.unwrap();
    process(socket).await;
  }
}


async fn process(socket: TcpStream) {
  let mut connect = Connection::new(socket);
  if let Some(frame) = connect.read_frame().await.unwrap() {
    println!("Got: {:?}", frame);
    let response = Frame::Error("err".to_string());
    connect.write_frame(&response).await.unwrap();
  }
}