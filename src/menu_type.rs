#[derive(Clone, Copy)]
pub enum MenuType {
    CUSTOM,
    DEFAULT,
}

pub trait MenuToStr {
    fn to_str<'a>(self) -> &'a str;
}

impl MenuToStr for MenuType {
    fn to_str<'a>(self) -> &'a str {
        match  self {
            MenuType::CUSTOM => "CUSTOM",
            MenuType::DEFAULT => "DEFAULT"
        }
    }
}