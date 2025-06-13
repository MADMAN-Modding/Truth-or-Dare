use serenity::all::{CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage};

/// Trait to convert into a footer
pub trait FooterMaker {
    fn to_footer(&self) -> CreateEmbedFooter;
}

pub trait MessageMaker {
    fn to_message(&self) -> CreateInteractionResponse;
}

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