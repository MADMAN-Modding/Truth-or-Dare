#[derive(Debug, sqlx::FromRow)]
pub struct GuildSettings {
    pub guild_id: i64,
    pub rating: String
}