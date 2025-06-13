use serenity::all::CreateEmbedFooter;

/// Trait to convert into a footer
pub trait FooterMaker {
    fn to_footer(&self) -> CreateEmbedFooter;
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