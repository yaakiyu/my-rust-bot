use crate::utils;

/// Echo back the given text.
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn ping(
    ctx: utils::Context<'_>,
    #[description = "The text to echo back"] text: String,
) -> utils::CommandResult {
    ctx.say(format!("Pong! {}", text)).await?;
    Ok(())
}

/// いいたいことを言う
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn say(
    ctx: utils::Context<'_>,
    #[description = "言いたいこと"] text: String,
) -> utils::CommandResult {
    ctx.say(format!("{}", text)).await?;
    Ok(())
}
