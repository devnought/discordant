use async_trait::async_trait;
use discordant_types::{
    Interaction, InteractionCallbackType, InteractionResponse, InteractionType,
};
use http::{HeaderMap, StatusCode};
use log::info;

use crate::{discord_verify, DiscordVerify, State};

#[async_trait]
pub trait DiscordHandler {
    async fn application_command(
        &self,
        _interaction: Interaction<'_>,
    ) -> Result<InteractionResponse, StatusCode> {
        Err(StatusCode::NOT_FOUND)
    }

    async fn message_component(
        &self,
        _interaction: Interaction<'_>,
    ) -> Result<InteractionResponse, StatusCode> {
        Err(StatusCode::NOT_FOUND)
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
}
