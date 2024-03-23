use poise::serenity_prelude as serenity;
use poise::FrameworkContext;
use serenity::{Context, FullEvent};

use crate::utils::{CommandResult, Data, Error};


pub async fn event_handler(
    ctx: &Context,
    event: &FullEvent,
    framework: FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> CommandResult {
    match event {
        FullEvent::Message { new_message } => {
            if new_message.content == "Hello, world!" && new_message.author.id != 712118236195323936 {
                new_message.channel_id.say(&ctx.http, "Hello, world!").await?;
            }
        }
        _ => {}
    }
    Ok(())
}
