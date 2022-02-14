use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command("end")]
#[only_in(guild)]
#[allowed_roles("Tournament Admin")]
#[description("Ends a tournament.")]
async fn end(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    todo!()
}

