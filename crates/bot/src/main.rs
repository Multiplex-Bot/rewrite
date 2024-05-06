mod commands;

use std::{env, time::Duration};

use axum::{routing::get, Router};
use commands::Data;
use poise::{
    serenity_prelude as serenity,
    serenity_prelude::{CacheHttp, GatewayIntents, GuildId},
};
use s3::{creds::Credentials, region::Region, Bucket};
use sqlx::PgPool;
use tokio::{task::JoinSet, time::sleep};

pub fn envvar(var: &str) -> String {
    env::var(var).expect(&format!(
        "Could not find {}; did you specify it in .env?",
        var
    ))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv()
        .expect("Could not find environment config; did you forget to `cp .env.template .env`?");

    let database_url = envvar("DATABASE_URL");
    let database = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database!");

    models::migrate(&database)
        .await
        .expect("Failed to run migrations!");

    let avatar_bucket = Bucket::new(
        &envvar("S3_AVATAR_BUCKET"),
        Region::Custom {
            region: envvar("S3_REGION"),
            endpoint: envvar("S3_ENDPOINT"),
        },
        Credentials::new(
            Some(&envvar("S3_KEY_ID")),
            Some(&envvar("S3_KEY_SECRET")),
            None,
            None,
            None,
        )
        .unwrap(),
    )
    .unwrap()
    .with_path_style();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::misc::ping(),
                commands::misc::support(),
                commands::misc::explain(),
            ],
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(async move {
                    match event {
                        _ => {}
                    }
                    Ok(())
                })
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                let create_commands =
                    poise::builtins::create_application_commands(&framework.options().commands);
                if let Ok(id) = env::var("DEV_GUILD") {
                    GuildId::new(id.parse::<u64>().unwrap())
                        .set_commands(ctx.http(), create_commands)
                        .await?;
                    tracing::info!("Using guild-specific slash commands in {}", id);
                } else {
                    poise::builtins::register_globally(ctx.http(), &framework.options().commands)
                        .await?;
                    tracing::info!(
                        "Using global slash commands; warning, this may take literally forever"
                    );
                }
                Ok(Data {
                    database,
                    avatar_bucket,
                })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(
        envvar("DISCORD_TOKEN"),
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT,
    )
    .framework(framework)
    .await
    .unwrap();

    let mut threads = JoinSet::new();

    threads.spawn(async move {
        client.start().await.unwrap();
    });

    threads.spawn(async move {
        let app = Router::new().route("/health", get(|| async { "( •̀ ω •́ )✧" }));

        let listener = tokio::net::TcpListener::bind(&envvar("HEALTH_CHECK_ADDRESS"))
            .await
            .unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    while !threads.is_empty() {
        sleep(Duration::from_secs(10)).await;
    }
}
