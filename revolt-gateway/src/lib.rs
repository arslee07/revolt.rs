use futures_util::{SinkExt, Stream, StreamExt};
use revolt_models::event::{ClientToServerEvent, ServerToClientEvent};
use std::{error::Error, time::Duration};
use tokio::{
    spawn,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    time::sleep,
};
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug, thiserror::Error)]
pub enum GatewayError {
    #[error("Tungstenite error: {0}")]
    WsError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Serde JSON deserialization/serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub struct RevoltWs {
    server_event_receiver: UnboundedReceiver<Result<ServerToClientEvent, GatewayError>>,
    client_event_sender: UnboundedSender<ClientToServerEvent>,
}

impl RevoltWs {
    pub async fn connect() -> Result<RevoltWs, GatewayError> {
        RevoltWs::connect_with_url("wss://ws.revolt.chat".to_string()).await
    }

    pub async fn connect_with_url(url: String) -> Result<Self, GatewayError> {
        let (ws, _) = tokio_tungstenite::connect_async(url).await?;
        let (mut ws_writer, mut ws_reader) = ws.split();

        let (server_sender, server_receiver) =
            mpsc::unbounded_channel::<Result<ServerToClientEvent, GatewayError>>();
        let (client_sender, mut client_receiver) = mpsc::unbounded_channel::<ClientToServerEvent>();

        let c = RevoltWs {
            server_event_receiver: server_receiver,
            client_event_sender: client_sender.clone(),
        };

        // Receive message -> decode -> send to channel
        spawn(async move {
            while let Some(msg) = ws_reader.next().await {
                // FIXME: what the fuck is this unreadable shit
                let event = match msg {
                    Ok(msg) => match msg.to_text() {
                        Ok(text) => match serde_json::from_str::<ServerToClientEvent>(text) {
                            Ok(res) => Ok(res),
                            Err(err) => Err(GatewayError::from(err)),
                        },
                        Err(err) => Err(GatewayError::from(err)),
                    },
                    Err(err) => Err(GatewayError::from(err)),
                };

                server_sender.send(event).unwrap();
            }
        });

        // Receive from channel -> encode -> send message
        spawn(async move {
            while let Some(event) = client_receiver.recv().await {
                let msg = Message::Text(serde_json::to_string(&event).unwrap());
                ws_writer.send(msg).await.unwrap();
            }
        });

        // Send heartbeat periodically
        spawn(async move {
            loop {
                client_sender
                    .send(ClientToServerEvent::Ping { data: 0 })
                    .unwrap();
                sleep(Duration::from_secs(15)).await;
            }
        });

        Ok(c)
    }

    pub async fn send(&mut self, event: ClientToServerEvent) -> Result<(), GatewayError> {
        self.client_event_sender.send(event).unwrap();

        Ok(())
    }
}

impl Stream for RevoltWs {
    type Item = Result<ServerToClientEvent, GatewayError>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.server_event_receiver.poll_recv(cx)
    }
}
