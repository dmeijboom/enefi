use std::collections::HashMap;

use anyhow::{anyhow, Result};

use super::Endpoints;

pub struct ClientInfo {
    pub version: String,
    pub client_id: String,
    pub client_secret: String,
    pub endpoints: Endpoints,
}

#[derive(Debug)]
enum Literal {
    Bool(bool),
    String(String),
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        match self {
            Literal::Bool(b) => b.to_string(),
            Literal::String(s) => s.clone(),
        }
    }
}

fn get_string(raw: &HashMap<String, Literal>, key: &str) -> Result<String> {
    raw.get(key)
        .map(Literal::to_string)
        .ok_or_else(|| anyhow!("missing key: '{}'", key))
}

fn parse_lit(mut input: &str) -> Result<Literal> {
    if input.is_empty() {
        return Err(anyhow!("empty valu"));
    }

    if input.ends_with(',') {
        input = &input[..input.len() - 1];
    }

    match input {
        "true" => Ok(Literal::Bool(true)),
        "false" => Ok(Literal::Bool(false)),
        input => {
            let mut output = String::new();
            let mut chars = input.chars();
            let delim = chars.next().unwrap();

            if !matches!(delim, '\'' | '"') {
                return Err(anyhow!("invalid value: {}", input));
            }

            loop {
                match chars.next() {
                    Some(c) if c == delim => break,
                    Some(c) => output.push(c),
                    None => return Err(anyhow!("unterminated string: {}", input)),
                }
            }

            Ok(Literal::String(output))
        }
    }
}

pub async fn get_client_info() -> Result<ClientInfo> {
    let response = reqwest::get("https://my.tado.com/webapp/env.js")
        .await?
        .text()
        .await?;

    let mut path = vec![];
    let mut raw = HashMap::new();

    for line in response.lines() {
        let is_object_start = line.contains('{');
        let is_object_end = line.contains('}');
        let is_key_val = line.contains(':');

        if !is_key_val {
            continue;
        }

        if is_object_start {
            let key = line
                .split(':')
                .next()
                .ok_or_else(|| anyhow!("invalid key"))?
                .trim();

            path.push(key);
        } else if is_object_end {
            if path.is_empty() {
                return Err(anyhow!("invalid path"));
            }

            path.pop();
        } else {
            let mut components = line.splitn(2, ':');
            let key = components
                .next()
                .ok_or_else(|| anyhow!("invalid key"))?
                .trim();
            let value = parse_lit(
                components
                    .next()
                    .ok_or_else(|| anyhow!("invalid value"))?
                    .trim(),
            )?;

            raw.insert(format!("{}.{}", path.join("."), key), value);
        }
    }

    Ok(ClientInfo {
        version: get_string(&raw, "config.version")?,
        client_id: get_string(&raw, "config.oauth.clientId")?,
        client_secret: get_string(&raw, "config.oauth.clientSecret")?,
        endpoints: Endpoints {
            oauth: get_string(&raw, "config.oauth.apiEndpoint")?,
        },
    })
}
