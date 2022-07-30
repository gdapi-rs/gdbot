use super::prelude::*;

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) -> Result<Message> {
	let client = ctx.data.read().await.get::<GdapiClientKey>().unwrap().clone();

	let id = match interaction.data.options[0]
		.value
		.as_ref()
		.unwrap()
		.as_i64()
		.unwrap()
		.try_into()
	{
		Ok(id) => id,
		Err(_) => return interaction.reply(&ctx, "Invalid id!").await,
	};

	let user = match client.user(id).await {
		Ok(user) => user,
		Err(_) => return interaction.reply(&ctx, "Invalid id!").await,
	};

	interaction.embed(&ctx, user::embed(&user)).await
}
