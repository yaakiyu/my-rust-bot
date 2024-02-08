# rust bot
Rustで作ったbotのあれこれ

## .envに書く内容
- `DISCORD_TOKEN` - Discord bot token
- `DATABASE_URL` - Database url

## コマンド追加方法
```rust
#[poise::command(prefix_command, slash_command)]
pub async fn コマンド名(
    ctx: utils::Context<'_>,
    #[description = "説明"] 引数名: 型,
) -> utils::CommandResult {
    ctx.say(内容).await?;
    Ok(())
}
```
を`commands/*.rs`ファイルに追加する。そして`commands/mod.rs`に
```rs
mod ファイル名;

use ファイル名::関数名;
```
を追加する。さらに、main.rsの`let framework = `あたりのコマンドを列挙してるところに
```rs
            commands: vec![commands::ping(), commands::introduction(), commands::say(), commands::関数名()],
```
みたいに追加すればOK

# 以下、rustのお勉強
## 型
- `i32` - 32bit整数
- `String` - 文字列
  - `format!("aaa {}", 5)` - `"aaa 5"`になる
