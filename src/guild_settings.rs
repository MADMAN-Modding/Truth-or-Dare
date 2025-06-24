use serenity::all::GuildId;

/// This struct defines the GuildSettings struct and its database representation.
/// I'm aware it is not used
#[derive(Debug, sqlx::FromRow)]
pub struct GuildSettings {
    pub guild_id: Option<GuildId>,
    pub rating: String,
    pub permissions: bool
}