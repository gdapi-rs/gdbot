use std::env;
use std::error::Error;

use serde_json::Value;
use serenity::http::Http;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	dotenv::dotenv()?;

	let application_id = env::var("APPLICATION_ID")?.parse()?;
	let token = env::var("TOKEN")?;

	let http = Http::new_with_application_id(&token, application_id);
	let data: Value = toml::from_str(include_str!("../../commands.toml"))?;
	let commands = data.get("commands").ok_or("toml file did not contain commands")?;

	http.create_global_application_commands(commands).await?;

	Ok(())
}
