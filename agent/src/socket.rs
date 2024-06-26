use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

const WS_ENDPOINT: &str = "ws://localhost:45000/ws";


pub(crate) async fn spawn_client() -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Error> {
    let socket = create_socket().await?;

    let (sender, receiver) = socket.split();

    //TODO Some info before execution

    Ok(sender.reunite(receiver).unwrap())
}


async fn create_socket() -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Error> {
    let ws_stream = match connect_async(WS_ENDPOINT).await {
        #[cfg(debug_assertions)]
        Ok((stream, response)) => {
            println!("Handshake for agent has been completed");
            println!("Server response was {response:?}");
            stream
        }
        #[cfg(not(debug_assertions))]
        Ok((stream, _response)) => {
            stream
        }
        Err(e) => {
            #[cfg(debug_assertions)]
            println!("WebSocket handshake for agent failed with {e}!");
            return Err(e);
        }
    };

    Ok(ws_stream)
}
