use std::{borrow::Cow, collections::HashMap};

use async_trait::async_trait;
use discordant_types::{
    ApplicationCommand, Interaction, InteractionCallbackType, InteractionResponse, InteractionType,
};
use http::{HeaderMap, StatusCode};
use log::info;

use crate::{discord_verify, DiscordVerify, State};

pub struct CommandHandler<'a>(
    pub ApplicationCommand<'a>,
    pub fn(Interaction) -> Result<InteractionResponse<'a>, StatusCode>,
);

#[async_trait]
pub trait DiscordHandler {
    fn application_commands(&self) -> &HashMap<Cow<'_, str>, CommandHandler>;
    fn command_list(&self) -> Vec<ApplicationCommand> {
        let commands = self.application_commands();

        let res = commands
            .values()
            .map(|CommandHandler(c, _)| c.clone())
            .collect::<Vec<_>>();

        res
    }

    async fn post_index(
        &self,
        state: &State,
        body: String,
        headers: HeaderMap,
    ) -> Result<InteractionResponse, StatusCode> {
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
                        self.application_command(interaction).await?
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

    async fn application_command(
        &self,
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

        let command_map = self.application_commands();

        let CommandHandler(_, handler) = command_map
            .get(name)
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        handler(interaction)
    }

    async fn message_component(
        &self,
        _interaction: Interaction<'_>,
    ) -> Result<InteractionResponse, StatusCode> {
        Err(StatusCode::NOT_FOUND)
    }
}
