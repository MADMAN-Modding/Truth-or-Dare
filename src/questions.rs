use std::fmt;
use std::str::FromStr;

use sqlx::{Decode, Sqlite, Type};
use sqlx::sqlite::SqliteValueRef;

/// Enum with implementations for the question type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestionType {
    TRUTH,
    DARE,
    NONE,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Question {
    pub id: i64,
    pub prompt: String,
    pub question_type: QuestionType,
    pub rating: String,
}

/// Get the question type as printable text
impl fmt::Display for QuestionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            QuestionType::TRUTH => "TRUTH",
            QuestionType::DARE => "DARE",
            QuestionType::NONE => "NONE",
        };
        write!(f, "{}", s)
    }
}

/// Get QuestionType from &str
impl FromStr for QuestionType {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_uppercase().as_str() {
            "TRUTH" => Ok(QuestionType::TRUTH),
            "DARE" => Ok(QuestionType::DARE),
            _ => Ok(QuestionType::NONE),
        }
    }
}

/// Get QuestionType from database
impl<'r> Decode<'r, sqlx::Sqlite> for QuestionType {
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as Decode<sqlx::Sqlite>>::decode(value)?;
        Ok(QuestionType::from_str(s.as_str())?)
    }
}

impl Type<Sqlite> for QuestionType {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}
