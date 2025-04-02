use anyhow::{Context, Result};
use grammers_client::Client;

pub fn save_session(client: &Client, session_file: &str) -> Result<()> {
    client
        .session()
        .save_to_file(session_file)
        .context("Failed to save session to file")
}
