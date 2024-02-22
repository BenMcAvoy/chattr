use bichannel::Channel;
use common::SourceMessage;

#[no_mangle]
pub extern "C" fn launch(plugin_channel: Box<Channel<SourceMessage, SourceMessage>>) {
    println!(">> Discord frontend launching.");

    plugin_channel
        .send(SourceMessage::Send(common::Message {
            identifier: "discord".to_string(),
            content: "Hello, world!".to_string(),
        }))
        .unwrap();
}
