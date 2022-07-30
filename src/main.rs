mod commands;
mod handler;
mod interaction;
mod util;

use std::env;
use std::error::Error;
use std::sync::Arc;

use gdapi::client::Client as GdapiClient;
use handler::Handler;
use serenity::client::Client as SerenityClient;
use serenity::model::gateway::GatewayIntents;
use serenity::prelude::TypeMapKey;

pub struct GdapiClientKey;

impl TypeMapKey for GdapiClientKey {
	type Value = Arc<GdapiClient>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	dotenv::dotenv()?;

	let token = env::var("TOKEN")?;

	let mut client = SerenityClient::builder(token, GatewayIntents::empty())
		.event_handler(Handler)
		.type_map_insert::<GdapiClientKey>(Arc::new(GdapiClient::new()))
		.await?;

	client.start().await?;

	Ok(())
}
