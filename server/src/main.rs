use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::interval;
use tokio::time::Duration;
use tokio_stream::wrappers::IntervalStream;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

async fn handle_connection(stream: TcpStream) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            println!("Error during the websocket handshake: {}", e);
            return;
        }
    };
    println!("WebSocket connection established");

    let (mut write, mut read) = ws_stream.split();

    // Interval for sending pings
    let mut interval = IntervalStream::new(interval(Duration::from_secs(5)));

    tokio::select! {
        _ = interval.next() => {
            if let Err(e) = write.send(Message::Ping(vec![])).await {
                println!("Failed to send ping: {}", e);
                return;
            }
        }
        msg = read.next() => match msg {
            Some(Ok(Message::Pong(_))) => {
                println!("Pong message received");
            }
            Some(Ok(Message::Close(_))) => {
                println!("Client disconnected");
                return;
            }
            Some(Err(e)) => {
                println!("Error receiving message: {}", e);
                return;
            }
            _ => {}
        },
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}
