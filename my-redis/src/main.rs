use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
  let list = TcpListener::bind("127.0.0.1:6379").await.unwrap();

  loop {
    let (i, j) = list.accept().await.unwrap();
    tokio::spawn(async move {process(i).await;});
  }
}

async fn process(socket: TcpStream) {
  let mut connect = Connection::new(socket);
  if let Some(frame) = connect.read_frame().await.unwrap() {
    println!("Got: {:?}", frame);
    let resp = Frame::Error("err".to_string());
    connect.write_frame(&resp).await.unwrap();
  }
}
