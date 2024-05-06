use anyhow::Result;

use super::CommandContext;

#[poise::command(slash_command, ephemeral)]
pub async fn edit(
    ctx: CommandContext<'_>,
    #[description = "the name of your collective"] name: Option<String>,
    #[description = "the bio of your collective"] bio: Option<String>,
    #[description = "the pronouns of your collective"] pronouns: Option<String>,
) -> Result<()> {
    ctx.say("This command is a work in progress!").await?;

    Ok(())
}
