use futures_util::{SinkExt, Stream, StreamExt};
use revolt_models::event::{ClientToServerEvent, ServerToClientEvent};
use std::{error::Error, time::Duration};
use tokio::{
    spawn,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    time::sleep,
};
use tokio_tungstenite::tungstenite::Message;

pub struct RevoltWs {
    server_event_receiver: UnboundedReceiver<ServerToClientEvent>,
    client_event_sender: UnboundedSender<ClientToServerEvent>,
}

impl RevoltWs {
    pub async fn connect() -> Result<RevoltWs, Box<dyn Error + Send + Sync>> {
        RevoltWs::connect_with_url("wss://ws.revolt.chat".to_string()).await
    }

    pub async fn connect_with_url(url: String) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let (ws, _) = tokio_tungstenite::connect_async(url).await?;
        let (mut ws_writer, mut ws_reader) = ws.split();

        let (server_sender, server_receiver) = mpsc::unbounded_channel::<ServerToClientEvent>();
        let (client_sender, mut client_receiver) = mpsc::unbounded_channel::<ClientToServerEvent>();

        let c = RevoltWs {
            server_event_receiver: server_receiver,
            client_event_sender: client_sender.clone(),
        };

        // Receive message -> decode -> send to channel
        spawn(async move {
            while let Some(msg) = ws_reader.next().await {
                let event =
                    serde_json::from_str::<ServerToClientEvent>(msg.unwrap().to_text().unwrap())
                        .unwrap();
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

    pub async fn send(
        &mut self,
        event: ClientToServerEvent,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.client_event_sender.send(event).unwrap();

        Ok(())
    }
}

impl Stream for RevoltWs {
    type Item = ServerToClientEvent;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.server_event_receiver.poll_recv(cx)
    }
}
