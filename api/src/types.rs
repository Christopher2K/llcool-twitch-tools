use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream};
use diesel::{r2d2, sqlite::SqliteConnection};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

pub type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
