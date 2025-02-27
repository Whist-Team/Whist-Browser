use std::fmt::Debug;

use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt, TryStreamExt};
use reqwest::{IntoUrl, Url};
use serde::Serialize;
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub enum WebSocketError {
    Connect,
    Serde(String),
    UnexpectedMessageType,
    WebSocket(String),
}

impl From<serde_json::Error> for WebSocketError {
    fn from(error: serde_json::Error) -> Self {
        WebSocketError::Serde(format!("{error}"))
    }
}

#[cfg(target_family = "wasm")]
impl From<gloo_net::websocket::WebSocketError> for WebSocketError {
    fn from(error: gloo_net::websocket::WebSocketError) -> Self {
        WebSocketError::WebSocket(format!("{}", error))
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
        WebSocketError::WebSocket(format!("{error:?}"))
    }
}

pub struct WebSocket;

impl WebSocket {
    fn convert_to_ws_url(url: impl IntoUrl) -> Url {
        let mut url = url.into_url().unwrap();
        match url.scheme() {
            "http" => url.set_scheme("ws").unwrap(),
            "https" => url.set_scheme("wss").unwrap(),
            "ws" | "wss" => {}
            other => panic!("unsupported url scheme '{other}'"),
        }

        url
    }
}

#[cfg(target_family = "wasm")]
impl WebSocket {
    pub async fn connect(
        url: impl IntoUrl,
    ) -> Result<(WebSocketSender, WebSocketReceiver), WebSocketError> {
        let websocket = gloo_net::websocket::futures::WebSocket::open(
            WebSocket::convert_to_ws_url(url).as_str(),
        )?;
        let (sink, stream) = websocket.split();
        Ok((WebSocketSender { sink }, WebSocketReceiver { stream }))
    }
}

#[cfg(not(target_family = "wasm"))]
impl WebSocket {
    pub async fn connect(
        url: impl IntoUrl,
    ) -> Result<(WebSocketSender, WebSocketReceiver), WebSocketError> {
        let (websocket, _) =
            tokio_tungstenite::connect_async(WebSocket::convert_to_ws_url(url)).await?;
        let (sink, stream) = websocket.split();
        Ok((WebSocketSender { sink }, WebSocketReceiver { stream }))
    }
}

pub struct WebSocketSender {
    #[cfg(target_family = "wasm")]
    sink: SplitSink<gloo_net::websocket::futures::WebSocket, gloo_net::websocket::Message>,

    #[cfg(not(target_family = "wasm"))]
    sink: SplitSink<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        tokio_tungstenite::tungstenite::protocol::Message,
    >,
}

impl WebSocketSender {
    pub async fn send_json<S: Serialize + Debug + ?Sized>(
        &mut self,
        data: &S,
    ) -> Result<(), WebSocketError> {
        self.send_text(serde_json::to_string(data)?).await
    }
}

#[cfg(target_family = "wasm")]
impl WebSocketSender {
    pub async fn send_text(&mut self, data: impl Into<String>) -> Result<(), WebSocketError> {
        self.sink
            .send(gloo_net::websocket::Message::Text(data.into()))
            .await?;
        Ok(())
    }
}

#[cfg(not(target_family = "wasm"))]
impl WebSocketSender {
    pub async fn send_text(
        &mut self,
        data: impl Into<tokio_tungstenite::tungstenite::protocol::Message>,
    ) -> Result<(), WebSocketError> {
        self.sink.send(data.into()).await?;
        Ok(())
    }
}

pub struct WebSocketReceiver {
    #[cfg(target_family = "wasm")]
    stream: SplitStream<gloo_net::websocket::futures::WebSocket>,

    #[cfg(not(target_family = "wasm"))]
    stream: SplitStream<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    >,
}

impl WebSocketReceiver {
    pub async fn recv_json<D: DeserializeOwned + Debug>(&mut self) -> Result<D, WebSocketError> {
        Ok(serde_json::from_str(self.recv_text().await?.as_str())?)
    }
}

#[cfg(target_family = "wasm")]
impl WebSocketReceiver {
    pub async fn recv_text(&mut self) -> Result<String, WebSocketError> {
        match self.stream.try_next().await? {
            Some(gloo_net::websocket::Message::Text(data)) => Ok(data),
            _ => Err(WebSocketError::UnexpectedMessageType),
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl WebSocketReceiver {
    pub async fn recv_text(&mut self) -> Result<String, WebSocketError> {
        match self.stream.try_next().await? {
            Some(tokio_tungstenite::tungstenite::protocol::Message::Text(data)) => {
                Ok(data.to_string())
            }
            _ => Err(WebSocketError::UnexpectedMessageType),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde::{Deserialize, Serialize};

    use crate::network::*;

    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    struct Test {
        my_cool_string: String,
    }

    /// only works with external websocket echo server on port 10000.
    ///
    /// try 'cargo run --example ws_echo_server' to start one
    // #[tokio::test]
    async fn test_ws_echo() {
        let data = Test {
            my_cool_string: "asdf".to_string(),
        };

        let (mut sender, mut receiver) = WebSocket::connect("ws://localhost:10000").await.unwrap();

        sender.send_json(&data).await.unwrap();
        let res: Test = receiver.recv_json().await.unwrap();

        assert_eq!(res, data);
    }
}
