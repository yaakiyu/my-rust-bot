use crate::utils;

/// 自分のレベルを確認する。
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn rank(
    ctx: utils::Context<'_>
) -> utils::CommandResult {
    ctx.say("レベル: 0\nExp: 0").await?;
    Ok(())
}
