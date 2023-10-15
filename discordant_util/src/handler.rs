use std::{borrow::Cow, collections::HashMap};

use discordant_types::{
    ApplicationCommand, Interaction, InteractionCallbackType, InteractionResponse, InteractionType,
};
use http::{HeaderMap, StatusCode};
use log::info;

use crate::{discord_verify, DiscordState, DiscordVerify};

#[derive(Debug)]
pub struct CommandHandler<'a, S>(
    pub ApplicationCommand<'a>,
    pub fn(&S, Interaction) -> Result<InteractionResponse<'a>, StatusCode>,
)
where
    S: DiscordState;

#[derive(Debug)]
pub struct DiscordHandler<'a, S>
where
    S: DiscordState,
{
    commands: HashMap<Cow<'a, str>, CommandHandler<'a, S>>,
}

impl<'a, S> DiscordHandler<'a, S>
where
    S: DiscordState,
{
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn command(mut self, value: CommandHandler<'a, S>) -> Self {
        let CommandHandler(command, handler) = value;

        self.commands
            .insert(command.name.clone(), CommandHandler(command, handler));

        self
    }

    pub fn command_list(&self) -> Vec<ApplicationCommand> {
        let res = self
            .commands
            .values()
            .map(|CommandHandler(c, _)| c.clone())
            .collect::<Vec<_>>();

        res
    }

    pub async fn post_index(
        &self,
        state: &S,
        body: String,
        headers: HeaderMap,
    ) -> Result<InteractionResponse, StatusCode>
    where
        S: DiscordState,
    {
        let verify = discord_verify(state, &body, headers);

        match verify {
            DiscordVerify::Invalid => Err(StatusCode::UNAUTHORIZED),
            DiscordVerify::Valid => {
                let interaction = match serde_json::from_str::<Interaction>(&body) {
                    Ok(i) => i,
                    Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
                };

                info!("{interaction:#?}");

                let res = match interaction.interaction_type {
                    InteractionType::Ping => InteractionResponse {
                        response_type: InteractionCallbackType::Pong,
                        data: None,
                    },
                    InteractionType::ApplicationCommand => {
                        self.application_command(state, interaction).await?
                    }
                    InteractionType::MessageComponent => {
                        self.message_component(interaction).await?
                    }
                    _ => {
                        unimplemented!(
                            "{:?} is not yet a supported message type",
                            interaction.interaction_type
                        );
                    }
                };

                Ok(res)
            }
        }
    }

    pub async fn application_command(
        &self,
        state: &S,
        interaction: Interaction<'_>,
    ) -> Result<InteractionResponse, StatusCode> {
        let data = interaction
            .data
            .as_ref()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        let name = data
            .name
            .as_ref()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        let CommandHandler(_, handler) = self
            .commands
            .get(name)
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        handler(state, interaction)
    }

    pub async fn message_component(
        &self,
        _interaction: Interaction<'_>,
    ) -> Result<InteractionResponse, StatusCode> {
        Err(StatusCode::NOT_FOUND)
    }
}
