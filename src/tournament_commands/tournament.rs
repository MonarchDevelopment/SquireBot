use std::fmt::format;

use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

use squire_core::tournament::TournamentPreset;

use crate::model::containers::{
    GuildAndTournamentIDMapContainer, GuildSettingsMapContainer, TournamentMapContainer,
    TournamentNameAndIDMapContainer,
};

use super::{
    admin_commands::admin::*,
    player_commands::{
        add_deck::*, confirm_result::*, decklist::*, decks::*, drop::*, list::*, match_result::*,
        name::*, ready::*, register::*, remove_deck::*,
    },
    settings_commands::*,
};

#[command("tournament")]
#[only_in(guild)]
#[aliases("tourn", "T", "t")]
#[sub_commands(
    admin,
    create,
    settings,
    add_deck,
    confirm_result,
    decklist,
    decks,
    drop,
    list,
    match_result,
    name,
    ready,
    register,
    remove_deck
)]
#[usage("!tournament <option>")]
#[description("Commands pretaining to tournaments.")]
async fn tournament(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    msg.reply(
        &ctx.http,
        "Please specify a subcommand. If you're unsure, use `!sb-help tournament`.",
    )
    .await?;
    Ok(())
}

#[command]
#[only_in(guild)]
#[usage("!tournament create <type>, <name>")]
#[example("`!tournament create swiss, 'New Tournament'`")]
#[example("`!t create fluid, 'New Tournament'`")]
#[allowed_roles("Tournament Admin")]
#[description("Adjust the settings of a specfic tournament.")]
async fn create(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Verify that the arguements are parsable and correct
    let preset = match args.single_quoted::<String>() {
        Err(_) => {
            msg.reply(
                &ctx.http,
                "Please include a tournament type, either 'swiss' or 'fluid'",
            )
            .await?;
            return Ok(());
        }
        Ok(s) => match s.to_lowercase().as_str() {
            "fluid" => TournamentPreset::Fluid,
            "swiss" => TournamentPreset::Swiss,
            _ => {
                msg.reply(
                    &ctx.http,
                    "Invalid tournament preset. The valid options are 'fluid' and 'swiss'.",
                )
                .await?;
                return Ok(());
            }
        },
    };
    let name = args.rest().trim().to_string();
    if name.is_empty() {
        msg.reply(&ctx.http, "Please include a name for the tournament.")
            .await?;
        return Ok(());
    }
    // Get the settings data
    let data = ctx.data.read().await;
    let all_settings = data.get::<GuildSettingsMapContainer>().unwrap();
    let guild: Guild = msg.guild(&ctx.cache).unwrap();
    let settings = all_settings.get_mut(&guild.id).unwrap();
    // Ensure that tournaments can be ran
    if !settings.is_configured() {
        msg.reply(
            &ctx.http, "This server isn't configured to run tournaments. Use the `!setup` command to help you with this.",
        )
            .await?;
        return Ok(());
    }
    println!("Server is configured");
    // Create the role that the tournament will be using
    let tourn_role = match guild
        .create_role(&ctx.http, |r| {
            r.mentionable(true).name(format!("{name} Player"))
        })
        .await
    {
        Ok(role) => role,
        Err(_) => {
            msg.reply(&ctx.http, "Unable to create a role for the tournament.")
                .await?;
            return Ok(());
        }
    };
    println!("Role created");
    // Create the tournament and store its data in the required places.
    // NOTE: `create_tournament` will only return an error if the server is not configured. We
    // already checked this, so we're safe to unwrap it.
    let tourn = settings
        .create_tournament(tourn_role.clone(), preset, name.clone())
        .unwrap();
    println!("Tournament created");
    let tourn_id = tourn.tourn.id.clone();
    let all_tourns = data.get::<TournamentMapContainer>().unwrap();
    all_tourns.insert(tourn_id.clone(), tourn);
    let mut name_and_id = data
        .get::<TournamentNameAndIDMapContainer>()
        .unwrap()
        .write()
        .await;
    name_and_id.insert(name, tourn_id.clone());
    let mut id_map = data
        .get::<GuildAndTournamentIDMapContainer>()
        .unwrap()
        .write()
        .await;
    if !id_map.contains_right(&guild.id) {
        id_map.insert_right(guild.id.clone());
    }
    id_map.insert_left(tourn_id, &guild.id);
    println!("Tourns: {:?}", all_tourns);
    println!("Names: {:?}", name_and_id);
    println!("Ids: {:?}", id_map);
    msg.reply(&ctx.http, "Tournament successfully created!")
        .await?;
    Ok(())
}

#[command]
#[only_in(guild)]
#[sub_commands(
    format,
    deck_count,
    require_checkin,
    require_deck,
    pairings,
    scoring,
    discord
)]
#[usage("!tournament settings <option>")]
#[allowed_roles("Tournament Admin")]
#[description("Adjust the settings of a specfic tournament.")]
async fn settings(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    msg.reply(
        &ctx.http,
        "Please specify a subcommand in order to adjust settings.",
    )
    .await?;
    Ok(())
}
