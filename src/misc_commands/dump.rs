use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::containers::{GuildSettingsMapContainer, TournamentMapContainer};

use std::{
    fs::{read_to_string, File},
    io::Write,
};

#[command("dump")]
#[owners_only]
#[description("Force saves all data.")]
async fn dump(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let tourns = data.get::<TournamentMapContainer>().unwrap();
    let settings = data.get::<GuildSettingsMapContainer>().unwrap();
    if let Ok(data) = serde_json::to_string(&**tourns) {
        if let Ok(mut file) = File::create("tournaments.json") {
            let r = write!(file, "{data}");
            if let Err(_) = r {
                msg.reply(
                    &ctx.http,
                    "Failed to write tournament data to file. **DATA NOT SAVED**.",
                )
                .await?;
            }
        } else {
            msg.reply(
                &ctx.http,
                "Failed to create tournament file. **DATA NOT SAVED**.",
            )
            .await?;
        }
    } else {
        msg.reply(
            &ctx.http,
            "Failed to serialize tournament. **DATA NOT SAVED**.",
        )
        .await?;
    }
    if let Ok(data) = serde_json::to_string(&**settings) {
        if let Ok(mut file) = File::create("guild_settings.json") {
            let r = write!(file, "{data}");
            if let Err(_) = r {
                msg.reply(
                    &ctx.http,
                    "Failed to write guild settings data to file. **DATA NOT SAVED**.",
                )
                .await?;
            }
        } else {
            msg.reply(
                &ctx.http,
                "Failed to create guild settings file. **DATA NOT SAVED**.",
            )
            .await?;
        }
    } else {
        msg.reply(
            &ctx.http,
            "Failed to serialize guild settings. **DATA NOT SAVED**.",
        )
        .await?;
    }
    Ok(())
}
