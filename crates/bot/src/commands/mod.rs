pub mod collective;
pub mod misc;

use anyhow::Error;
use s3::Bucket;
use sqlx::PgPool;

pub struct Data {
    pub database: PgPool,
    pub avatar_bucket: Bucket,
}
pub type CommandContext<'a> = poise::Context<'a, Data, Error>;
