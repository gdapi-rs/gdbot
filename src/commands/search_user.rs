use super::prelude::*;

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<Message> {
	let client = ctx.data.read().await.get::<GdapiClientKey>().unwrap().clone();

	let query = interaction.data.options[0].value.as_ref().unwrap().as_str().unwrap();

	let user = match client.search_user(query).await {
		Ok(user) => user,
		Err(_) => return interaction.reply(&ctx, "Could not find a user!").await,
	};

	interaction.embed(&ctx, user::embed(&user)).await
}
