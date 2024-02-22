use azalea::prelude::*;
use bichannel::Channel;
use common::{Message, SourceMessage};
use tokio::runtime::Runtime;

use std::env;

// const SERVER_IP: &str = "simpvp.net";
const SERVER_IP: &str = "localhost";

#[no_mangle]
pub extern "C" fn launch(plugin_channel: Box<Channel<SourceMessage, SourceMessage>>) {
    println!(">> Minecraft frontend launching.");

    // if env::var("RUST_LOG").is_err() {
    //     env::set_var("RUST_LOG", "tracing");
    // }

    // println!(">> Set log level to error.");

    Runtime::new().unwrap().block_on(inner(*plugin_channel));
}

async fn inner(channel: Channel<SourceMessage, SourceMessage>) {
    dotenvy::dotenv().unwrap();

    let email = env::var("MS_EMAIL").unwrap();

    // let account = Account::offline("bot");
    let account = Account::microsoft(&email).await.unwrap();

    let channel = Some(channel);

    println!(">> Minecraft client launching");

    ClientBuilder::new()
        .set_handler(handle)
        .set_state(State { channel })
        .start(account.clone(), SERVER_IP)
        .await
        .unwrap();
}

#[derive(Default, Clone, Component)]
pub struct State {
    channel: Option<Channel<SourceMessage, SourceMessage>>,
}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    // match event {
    //     Event::Chat(m) => {
    if let Event::Chat(m) = event {
        if let Some(username) = m.username() {
            if username == bot.username() {
                return Ok(());
            }

            let (identifier, content) = (m.username(), m.content());
            if let (Some(identifier), content) = (identifier, content) {
                let message = SourceMessage::Send(Message {
                    identifier,
                    content,
                });

                state.channel.unwrap().send(message).unwrap();
            }
        }

        // println!("{}", m.message().to_ansi());
    }
    // _ => {}
    // }

    Ok(())
}
