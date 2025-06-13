use serenity::all::{CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage};

/// Trait to convert into a footer
pub trait FooterMaker {
    fn to_footer(&self) -> CreateEmbedFooter;
}

/// Trait to convert into a messages
pub trait MessageMaker {
    fn to_message(&self) -> CreateInteractionResponse;
}

// Implementations for FooterMaker and MessageMaker traits for &str and String types
impl FooterMaker for &str {
    fn to_footer(&self) -> CreateEmbedFooter {
        CreateEmbedFooter::new(*self)
    }
}

impl FooterMaker for String {
    fn to_footer(&self) -> CreateEmbedFooter {
        CreateEmbedFooter::new(self.as_str())
    }
}

/// Implementations for MessageMaker trait for &str and String types
impl MessageMaker for &str {
    fn to_message(&self) -> CreateInteractionResponse {
        CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(*self))
    }
}

impl MessageMaker for String {
    fn to_message(&self) -> CreateInteractionResponse {
        CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(self.as_str()))
    }
}