use serenity::all::{
    CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage,
};

/// Trait to convert into a footer
pub trait FooterMaker {
    fn to_footer(&self) -> CreateEmbedFooter;
}

/// Trait to convert into a messages
pub trait MessageMaker {
    fn to_interaction_message(&self) -> CreateInteractionResponse;
    fn to_message(&self) -> CreateMessage;
}

/// Trait to convert to embed
pub trait EmbedMaker {
    fn to_embed(&self, title: impl AsRef<str>, footer: impl AsRef<str>) -> CreateEmbed;
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
    fn to_interaction_message(&self) -> CreateInteractionResponse {
        CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(*self))
    }

    fn to_message(&self) -> CreateMessage {
        CreateMessage::new().content(*self)
    }
}

impl MessageMaker for String {
    fn to_interaction_message(&self) -> CreateInteractionResponse {
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(self.as_str()),
        )
    }

    fn to_message(&self) -> CreateMessage {
        CreateMessage::new().content(self.as_str())
    }
}

impl MessageMaker for CreateEmbed {
    fn to_interaction_message(&self) -> CreateInteractionResponse {
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().embed(self.clone()),
        )
    }

    fn to_message(&self) -> CreateMessage {
        CreateMessage::new().embed(self.clone())
    }
}

// impl MessageMaker for CreateMessage {
//     fn to_interaction_message(&self) -> CreateInteractionResponse {

        
//         // CreateInteractionResponse::Message(
//         //     CreateInteractionResponseMessage::new().content(self).embed(self.embeds.clone())
//         // )
//     }

//     fn to_message(&self) -> CreateMessage {
//         self.clone()
//     }
// }

/// Implementations for EmbedMaker trait for &str and String types
impl EmbedMaker for &str {
    fn to_embed(&self, title: impl AsRef<str>, footer: impl AsRef<str>) -> CreateEmbed {
        CreateEmbed::new()
            .title(title.as_ref())
            .description(*self)
            .footer(CreateEmbedFooter::new(footer.as_ref()))
    }
}

impl EmbedMaker for String {
    fn to_embed(&self, title: impl AsRef<str>, footer: impl AsRef<str>) -> CreateEmbed {
        CreateEmbed::new()
            .title(title.as_ref())
            .description(self.as_str())
            .footer(CreateEmbedFooter::new(footer.as_ref()))
    }
}
