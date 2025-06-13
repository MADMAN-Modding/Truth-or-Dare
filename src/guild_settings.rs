/// This struct defines the GuildSettings struct and its database representation.
#[derive(Debug, sqlx::FromRow)]
pub struct GuildSettings {
    pub guild_id: i64,
    pub rating: String
}