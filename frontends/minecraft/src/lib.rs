use azalea::{prelude::*, FormattedText};
use std::{env, sync::{mpsc::Sender, Arc}};
use tokio::runtime::Runtime;

const SERVER_IP: &str = "simpvp.net";


#[no_mangle]
pub extern "C" fn launch(sender: Box<Sender<FormattedText>>) {
    println!("We launched the frontend.");

    Runtime::new().unwrap().block_on(inner(*sender));
}

async fn inner(sender: Sender<FormattedText>) {
    dotenvy::dotenv().unwrap();

    let email = env::var("MS_EMAIL").unwrap();

    // let account = Account::offline("bot");
    let account = Account::microsoft(&email).await.unwrap();

    let sender = Some(Arc::new(sender));

    ClientBuilder::new()
        .set_handler(handle)
        .set_state(State { sender })
        .start(account.clone(), SERVER_IP)
        .await
        .unwrap();
}

#[no_mangle]
pub fn hello() {
    println!("Hello from minecraft frontend.");
}

#[derive(Default, Clone, Component)]
pub struct State {
    sender: Option<Arc<Sender<FormattedText>>>
}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    // match event {
    //     Event::Chat(m) => {
    if let Event::Chat(m) = event {
        if let Some(username) = m.username() {
            if username == bot.username() {
                return Ok(());
            }

            state.sender.as_ref().unwrap().send(m.message()).unwrap();
        }

        tracing::info!("{}", m.message().to_ansi());
    }
    // _ => {}
    // }

    Ok(())
}
