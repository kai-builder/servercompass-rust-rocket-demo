#[macro_use]
extern crate rocket;

use rocket::response::content::RawHtml;
use rocket::serde::json::Json;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct EnvItem {
    key: &'static str,
    value: String,
}

#[derive(Serialize)]
struct EnvResponse {
    envs: Vec<EnvItem>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

const PUBLIC_KEYS: [&str; 4] = ["APP_NAME", "API_URL", "ENVIRONMENT", "VERSION"];
const HTML_TEMPLATE: &str = include_str!("./template.html");

fn default_for(key: &'static str) -> Option<&'static str> {
    match key {
        "APP_NAME" => Some("ServerCompass Rust Rocket"),
        "API_URL" => Some("https://api.servercompass.app"),
        "ENVIRONMENT" => Some("production"),
        "VERSION" => Some("1.0.0"),
        "DATABASE_URL" => Some("postgresql://user:password@localhost:5432/servercompass"),
        "API_SECRET_KEY" => Some("your-secret-key-here"),
        _ => None,
    }
}

fn env_value(key: &'static str) -> String {
    match env::var(key) {
        Ok(v) => {
            let trimmed = v.trim();
            if trimmed.is_empty() {
                "Not set".to_string()
            } else {
                trimmed.to_string()
            }
        }
        Err(_) => default_for(key).unwrap_or("Not set").to_string(),
    }
}

fn public_envs() -> Vec<EnvItem> {
    PUBLIC_KEYS
        .iter()
        .map(|&key| EnvItem {
            key,
            value: env_value(key),
        })
        .collect()
}

#[get("/api/env")]
fn api_env() -> Json<EnvResponse> {
    Json(EnvResponse { envs: public_envs() })
}

#[get("/")]
fn index() -> RawHtml<String> {
    let app_name = env_value("APP_NAME");
    let api_url = env_value("API_URL");
    let environment = env_value("ENVIRONMENT");
    let version = env_value("VERSION");

    let class_for = |v: &str| if v == "Not set" { "not-set" } else { "" };

    let rendered = HTML_TEMPLATE
        .replace("{{APP_NAME}}", &app_name)
        .replace("{{API_URL}}", &api_url)
        .replace("{{ENVIRONMENT}}", &environment)
        .replace("{{VERSION}}", &version)
        .replace("{{APP_NAME_CLASS}}", class_for(&app_name))
        .replace("{{API_URL_CLASS}}", class_for(&api_url))
        .replace("{{ENVIRONMENT_CLASS}}", class_for(&environment))
        .replace("{{VERSION_CLASS}}", class_for(&version));

    RawHtml(rendered)
}

#[get("/health")]
fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "servercompass-rust-rocket-demo",
    })
}

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok();

    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000);

    rocket::build()
        .configure(rocket::Config {
            address: std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
            port,
            ..rocket::Config::default()
        })
        .mount("/", routes![index, api_env, health])
}
