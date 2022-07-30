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

	let level = match client.level(id).await {
		Ok(level) => level,
		Err(_) => return interaction.reply(&ctx, "Invalid id!").await,
	};

	let mut description = if level.description.is_empty() {
		String::new()
	} else {
		format!("{}\n\n", level.description)
	};

	write!(
		description,
		"**Author Id:** {}\n\
		**Difficulty:** {}\n\
		**Rating:** {}\n\
		**Copy Type:** {}",
		level.author_id,
		level::difficulty(&level),
		level::rating(&level),
		level::copy_type(&level),
	)
	.unwrap();

	let mut embed = CreateEmbed::default();

	embed
		.title(format!("{} ({})", level.name, level.id))
		.thumbnail(level::thumbnail(&level))
		.description(description);

	interaction.embed(&ctx, embed).await
}
