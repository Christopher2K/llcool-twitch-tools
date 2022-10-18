use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream};

pub type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
