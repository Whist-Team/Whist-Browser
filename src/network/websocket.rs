use std::fmt::Debug;

use futures::{SinkExt, TryStreamExt};
use reqwest::{IntoUrl, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug)]
pub enum WebSocketError {
    Connect,
    Serde(String),
    UnexpectedMessageType,
    WebSocketError(String),
}

impl From<serde_json::Error> for WebSocketError {
    fn from(error: serde_json::Error) -> Self {
        WebSocketError::Serde(format!("{}", error))
    }
}

#[cfg(target_family = "wasm")]
impl From<gloo_net::websocket::WebSocketError> for WebSocketError {
    fn from(error: gloo_net::websocket::WebSocketError) -> Self {
        WebSocketError::WebSocketError(format!("{}", error))
    }
}

#[cfg(target_family = "wasm")]
impl From<gloo_utils::errors::JsError> for WebSocketError {
    fn from(_: gloo_utils::errors::JsError) -> Self {
        WebSocketError::Connect
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<tokio_tungstenite::tungstenite::Error> for WebSocketError {
    fn from(error: tokio_tungstenite::tungstenite::Error) -> Self {
        WebSocketError::WebSocketError(format!("{:?}", error))
    }
}

pub struct WebSocketConnection {
    #[cfg(target_family = "wasm")]
    websocket: gloo_net::websocket::futures::WebSocket,

    #[cfg(not(target_family = "wasm"))]
    websocket: tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
}

impl WebSocketConnection {
    pub async fn send_json<S: Serialize + Debug + ?Sized>(
        &mut self,
        data: &S,
    ) -> Result<(), WebSocketError> {
        self.send_text(serde_json::to_string(data)?).await
    }

    pub async fn recv_json<D: DeserializeOwned + Debug>(&mut self) -> Result<D, WebSocketError> {
        Ok(serde_json::from_str(self.recv_text().await?.as_str())?)
    }

    fn convert_to_ws_url(url: impl IntoUrl) -> Url {
        let mut url = url.into_url().unwrap();
        match url.scheme() {
            "http" => url.set_scheme("ws").unwrap(),
            "https" => url.set_scheme("wss").unwrap(),
            "ws" | "wss" => {}
            other => panic!("unsupported url scheme '{}'", other),
        }

        url
    }
}

#[cfg(target_family = "wasm")]
impl WebSocketConnection {
    pub async fn new(url: impl IntoUrl) -> Result<Self, WebSocketError> {
        let websocket = gloo_net::websocket::futures::WebSocket::open(
            WebSocketConnection::convert_to_ws_url(url).as_str(),
        )?;
        Ok(WebSocketConnection { websocket })
    }

    pub async fn send_text(&mut self, data: impl Into<String>) -> Result<(), WebSocketError> {
        self.websocket
            .send(gloo_net::websocket::Message::Text(data.into()))
            .await?;
        Ok(())
    }

    pub async fn recv_text(&mut self) -> Result<String, WebSocketError> {
        match self.websocket.try_next().await?.unwrap() {
            gloo_net::websocket::Message::Text(data) => Ok(data),
            _ => Err(WebSocketError::UnexpectedMessageType),
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl WebSocketConnection {
    pub async fn connect(url: impl IntoUrl) -> Result<Self, WebSocketError> {
        let (websocket, _) =
            tokio_tungstenite::connect_async(WebSocketConnection::convert_to_ws_url(url)).await?;
        Ok(WebSocketConnection { websocket })
    }

    pub async fn send_text(&mut self, data: impl Into<String>) -> Result<(), WebSocketError> {
        self.websocket
            .send(tokio_tungstenite::tungstenite::protocol::Message::Text(
                data.into(),
            ))
            .await?;
        Ok(())
    }

    pub async fn recv_text(&mut self) -> Result<String, WebSocketError> {
        match self.websocket.try_next().await?.unwrap() {
            tokio_tungstenite::tungstenite::protocol::Message::Text(data) => Ok(data),
            _ => Err(WebSocketError::UnexpectedMessageType),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::network::*;

    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    struct Test {
        my_cool_string: String,
    }

    /// only works with external websocket echo server on port 10000.
    ///
    /// try 'docker run -p 10000:8080 jmalloc/echo-server' to start one
    // #[tokio::test]
    async fn test_ws_echo() {
        let data = Test {
            my_cool_string: "asdf".to_string(),
        };

        let mut ws = WebSocketConnection::connect("ws://localhost:10000")
            .await
            .unwrap();
        ws.recv_text().await.unwrap(); // ignore echo response from connect

        ws.send_json(&data).await.unwrap();
        let res: Test = ws.recv_json().await.unwrap();

        assert_eq!(res, data);
    }
}
