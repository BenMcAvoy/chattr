use std::{
    collections::HashMap,
    env,
    ffi::OsStr,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use bichannel::Channel;
use common::SourceMessage;

type SourceLaunchFunction = fn(
    Box<Channel<SourceMessage, SourceMessage>>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>;

// #[derive(Debug)]
// struct Source {
//     channel: Channel<SourceMessage, SourceMessage>,
// }

// impl Source {
//     pub fn new(channel: Channel<SourceMessage, SourceMessage>) -> Self {
//         Self { channel }
//     }
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[allow(unused_variables)]
    #[cfg(debug_assertions)]
    let mut frontends_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    frontends_dir.pop();
    frontends_dir.push("target/debug/");

    #[cfg(not(debug_assertions))]
    let frontends_dir = "./frontends";

    let map: HashMap<String, Channel<SourceMessage, SourceMessage>> = HashMap::new();
    let map = Arc::new(Mutex::new(map));

    let files: Vec<_> = fs::read_dir(&frontends_dir)?
        .map(|f| f.unwrap().path())
        .filter(|path| {
            path.extension()
                .and_then(OsStr::to_str)
                .map_or(false, |ext| ext == "dll" || ext == "so")
        })
        .collect();

    for file in files {
        if let Some(filename) = file.file_name() {
            let filename_str = filename.to_string_lossy().to_string();
            println!("Found frontend `{filename_str}`");

            if filename_str == "libdiscord.so" {
                continue;
            }

            let lib = unsafe { libloading::Library::new(&file)? };

            let (l, r) = bichannel::channel();
            map.lock().unwrap().insert(filename_str, r);

            tokio::task::spawn(async move {
                let startup: libloading::Symbol<SourceLaunchFunction> =
                    unsafe { lib.get(b"launch").unwrap() };

                startup(Box::new(l)).await;
            });
        }
    }

    // println!("Sleeping for 10 seconds");
    // thread::sleep(Duration::from_secs(10));
    // println!("Done sleeping");

    loop {
        let map = map.lock().unwrap();
        let channel = map.get("libminecraft.so").unwrap();

        while let Ok(event) = channel.try_recv() {
            println!("Got event: {event:?}");
        }
    }
}
