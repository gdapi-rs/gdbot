use gdapi::model::user::User;
use serenity::builder::CreateEmbed;

pub fn embed(user: &User) -> CreateEmbed {
	let description = format!("**Account Id:** {}", user.account_id);

	let mut embed = CreateEmbed::default();

	embed
		.title(format!("{} ({})", user.username, user.id))
		.description(description);

	embed
}
