use std::{env, error::Error, sync::Arc};

use futures_util::StreamExt;

use revolt::{
    gateway::RevoltWs,
    http::RevoltHttp,
    models::{
        authentication::Authentication,
        event::{ClientToServerEvent, ServerToClientEvent},
        message::Message,
        payload::SendMessagePayload,
    },
    util::extensions::BuilderExt,
};

// Context stores important things like revolt client or database connection
struct Context {
    http: RevoltHttp,
}

type Result<T = ()> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result {
    // Get token from environment variable
    let token = env::var("TOKEN")?;

    // Create an http and ws instances
    let http = RevoltHttp::new(Authentication::BotToken(token.clone()));
    let mut ws = RevoltWs::connect().await?;

    // Send authentication event to start receiving events
    ws.send(ClientToServerEvent::Authenticate {
        token: token.clone(),
    })
    .await?;

    // Create the context
    let ctx = Arc::new(Context { http });

    // Wait for new event and then handle it
    while let Some(event) = ws.next().await {
        let event = event?;
        handle_event(event, Arc::clone(&ctx)).await?;
    }

    Ok(())
}

async fn handle_event(event: ServerToClientEvent, ctx: Arc<Context>) -> Result {
    match event {
        ServerToClientEvent::Message { message } => {
            handle_message(message, Arc::clone(&ctx)).await?;
        }
        ServerToClientEvent::Authenticated => {
            println!("Client is authenticated");
        }
        _ => {}
    };

    Ok(())
}

async fn handle_message(message: Message, ctx: Arc<Context>) -> Result {
    if let Some(ref content) = message.content {
        match content.as_str() {
            "!ping" => ping(message, Arc::clone(&ctx)).await?,
            _ => {}
        };
    }

    Ok(())
}

async fn ping(message: Message, ctx: Arc<Context>) -> Result {
    let payload = SendMessagePayload::builder().content("pong").build();
    ctx.http.send_message(message.channel, payload).await?;

    Ok(())
}
