use crate::utils;
use db::leveling as db;

/// 自分のレベルを確認する。
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn rank(
    ctx: utils::Context<'_>
) -> utils::CommandResult {
    let exp = db::get_level(&ctx.data().pool, ctx.author().clone().id.get() as i64).await?;
    let level = (exp as f64 / 10.0).sqrt() as i64;
    ctx.say(format!(
        "レベル: {}\nExp: {}",
        level.to_string(), exp.to_string()
    )).await?;
    Ok(())
}
