use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::channel::Message;
use serenity::{async_trait, Result};

#[async_trait]
pub trait ApplicationCommandInteractionExt {
	async fn embed(&self, ctx: &Context, embed: CreateEmbed) -> Result<Message>;
	async fn reply(&self, ctx: &Context, content: &str) -> Result<Message>;
}

#[async_trait]
impl ApplicationCommandInteractionExt for ApplicationCommandInteraction {
	async fn embed(&self, ctx: &Context, embed: CreateEmbed) -> Result<Message> {
		self.create_followup_message(ctx, |data| data.set_embed(embed)).await
	}

	async fn reply(&self, ctx: &Context, content: &str) -> Result<Message> {
		self.create_followup_message(ctx, |data| data.content(content)).await
	}
}
