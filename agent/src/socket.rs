use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

const WS_ENDPOINT: &str = "ws://0.0.0.0:3000/ws";


pub(crate) async fn spawn_client() -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Error> {
    let socket = create_socket().await?;

    let (sender, receiver) = socket.split();

    //TODO Some info before execution

    Ok(sender.reunite(receiver).unwrap())
}


async fn create_socket() -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Error> {
    let ws_stream = match connect_async(WS_ENDPOINT).await {
        Ok((stream, response)) => {
            println!("Handshake for client has been completed");
            println!("Server response was {response:?}");
            stream
        }
        Err(e) => {
            println!("WebSocket handshake for client failed with {e}!");
            return Err(e);
        }
    };

    Ok(ws_stream)
}