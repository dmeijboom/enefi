mod tado;

use std::env;

use anyhow::Result;
use clap::Parser;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::tado::{Client, GrantType, RequestTokenRequest};

#[derive(Parser)]
struct Opts {
    #[clap(long, env = "TADO_USERNAME")]
    tado_username: String,
    #[clap(long, env = "TADO_PASSWORD")]
    tado_password: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let opts = Opts::parse();
    let client_info = tado::get_client_info().await?;

    tracing::info!(version = client_info.version, "tado client");

    let client = Client::new(client_info.endpoints);
    let response = client
        .oauth()
        .request_token(&RequestTokenRequest {
            client_id: client_info.client_id,
            client_secret: client_info.client_secret,
            grant_type: GrantType::Password,
            scope: "home.user".to_string(),
            username: opts.tado_username,
            password: opts.tado_password,
        })
        .await?;

    println!("{:#?}", response);

    Ok(())
}
