/// Source message types.
#[derive(Debug)]
pub enum SourceMessage {
    Send(Message),
    Control(SourceControl),
}

/// Source control message types for influencing the source control flow.
#[derive(Debug)]
pub enum SourceControl {
    Start,
    Stop,
}

/// Used universally across all frontends for aggregation.
#[derive(Debug)]
pub struct Message {
    pub identifier: String,
    pub content: String,
}
