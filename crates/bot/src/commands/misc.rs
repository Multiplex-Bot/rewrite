use anyhow::Result;
use chrono::Utc;
use poise::{serenity_prelude::CreateEmbed, CreateReply};

use super::CommandContext;
use crate::envvar;

/// Ping pong 🏓
#[poise::command(slash_command, ephemeral)]
pub async fn ping(ctx: CommandContext<'_>) -> Result<()> {
    ctx.say(format!(
        "Pong :3 ({}ms)",
        (Utc::now().time() - ctx.created_at().time()).num_milliseconds()
    ))
    .await?;

    Ok(())
}

/// Join the support server!
#[poise::command(slash_command, ephemeral)]
pub async fn support(ctx: CommandContext<'_>) -> Result<()> {
    ctx.say(format!(
        "Join the support & discussion server at {}!", // fucked up
        envvar("SUPPORT_INVITE")
    ))
    .await?;

    Ok(())
}

/// Explains the purpose of the bot, and provides further links for more information
#[poise::command(slash_command)]
pub async fn explain(ctx: CommandContext<'_>) -> Result<()> {
    let embed = CreateEmbed::new().fields(vec![
        ("What is Multiplex?", "Multiplex is a \"message proxying\" bot that allows people to send messages as webhooks with custom profile pictures, names, etc.", false),
        ("Why is this used?", "Generally, these bots are used for either plural systems to identify who's talking, or roleplaying.", false),
        ("What is plurality?", "TL;DR: it's the experience of having multiple personalities in one body. (This is a very over-simplified explanation, please see https://morethanone.info for a better definition.)", false),
        ("Why are the bots talking?", "Discord shows webhooks as bots. No, they aren't real bots.", false)
    ]);

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}
