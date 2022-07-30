use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;

use crate::commands;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		let interaction = interaction.application_command().unwrap();

		interaction.defer(&ctx.http).await.unwrap();

		let result = match interaction.data.name.as_str() {
			"level" => commands::level::run(ctx, interaction).await,
			"search-user" => commands::search_user::run(ctx, interaction).await,
			"user" => commands::user::run(ctx, interaction).await,
			_ => return,
		};

		result.unwrap();
	}

	async fn ready(&self, _: Context, ready: Ready) {
		println!("{} is ready!", ready.user.name);
	}
}
