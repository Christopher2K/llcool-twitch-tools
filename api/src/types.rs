use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream};
use diesel::{r2d2, pg::PgConnection};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
