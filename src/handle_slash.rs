use anyhow::Result;
use autometrics::autometrics;
use serenity::{model::prelude::interaction::application_command::ApplicationCommandInteraction, prelude::Context};

#[autometrics]
pub async fn handle_slash(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<()> {
    let content = match interaction.data.name.as_str() {
                        "help" => "You have to right click a voice message and then select `Apps -> Transcribe Message` to use this bot for more info visit the [Github Repo](<https://github.com/Tricked-dev/Speech-To-Text-Bot>)",
                        "invite" => "You can invite the bot with [this link](https://discord.com/oauth2/authorize?client_id=838065007971139594&scope=bot%20applications.commands&permissions=0)",
                        "terms" => "<https://github.com/Tricked-dev/Speech-To-Text-Bot/blob/master/TERMS.md>",
                        "privacy" => "<https://github.com/Tricked-dev/Speech-To-Text-Bot/blob/master/PRIVACY.md>",
                        _ => "Unknown Command"
                    };
    interaction
        .edit_original_interaction_response(&ctx.http, |response| response.content(content).allowed_mentions(|x| x))
        .await?;
    Ok(())
}
