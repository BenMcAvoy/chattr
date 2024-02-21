use std::{env, path::PathBuf, sync::mpsc::{channel, Sender}};
use notify::{Watcher, RecursiveMode, Event};
use libloading::{Library, Symbol};
use azalea::FormattedText;
use tokio::runtime::Runtime;

type SourceLaunchFunction = fn(Box<Sender<FormattedText>>) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[allow(unused_variables)]
    #[cfg(debug_assertions)]
    let mut watch_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    watch_dir.pop();
    watch_dir.push("target/debug/");
    #[cfg(not(debug_assertions))]
    let watch_dir = "./frontends";

    let mut watcher = notify::recommended_watcher(|res: std::result::Result<Event, _>| {
        match res {
            Ok(event) => {
                let path = match event.paths.first() {
                    Some(path) => path,
                    None => return,
                };

                if let Some(ext) = path.extension() {
                    if ext != "dll" && ext != "so" {
                        return;
                    }
                } else {
                    return;
                }

                if let Some(path_str) = path.to_str() {
                    if path_str.contains("deps") {
                        return;
                    }
                }

                unsafe {
                    println!("Loading {}", path.to_str().unwrap());

                    match Library::new(path) {
                        Ok(lib) => {
                            println!("Loaded {}", path.to_str().unwrap());

                            lib.get::<Symbol<fn()>>(b"hello").expect("Could not get startup function")();

                            let startup: Symbol<SourceLaunchFunction> = lib.get(b"launch").expect("Could not get startup function");
                            Runtime::new().unwrap().block_on(startup(Box::new(channel().0)));
                        },
                        Err(e) => {
                            eprintln!("Failed to load library: {}", e);
                        }
                    }

                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    })?;

    watcher.watch(&watch_dir, RecursiveMode::Recursive)?;

    loop {
        println!("Looping");
        std::thread::park();
    }
}
