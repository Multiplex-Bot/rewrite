use rust_decimal::{prelude::FromPrimitive, Decimal};
use sqlx::{postgres::PgConnection, types::Uuid, FromRow};

use crate::mate::Mate;

#[derive(FromRow, Default)]
pub struct Collective {
    pub id: Uuid,
    pub user_id: Vec<Decimal>,
    pub name: String,
    pub description: String,
    pub pronouns: String,
    pub banner_url: String,
    pub thumbnail_url: String,
    pub collective_tag: [Option<String>; 2],
}

impl Collective {
    pub async fn get_from_user_id(user_id: u128, mut conn: PgConnection) -> Option<Collective> {
        sqlx::query_as("SELECT * FROM collectives WHERE user_id @> ?::numeric[]")
            .bind(Decimal::from_u128(user_id))
            .fetch_optional(&mut conn)
            .await
            .unwrap()
    }

    pub async fn get_mates(&self, mut conn: PgConnection) -> Vec<Mate> {
        sqlx::query_as("SELECT * FROM mates WHERE collective_id = ?")
            .bind(self.id)
            .fetch_all(&mut conn)
            .await
            .unwrap()
    }

    pub async fn get_mate_by_name(&self, name: &str, mut conn: PgConnection) -> Option<Mate> {
        sqlx::query_as("SELECT * FROM mates WHERE collective_id = ? AND name = ?")
            .bind(self.id)
            .bind(name)
            .fetch_optional(&mut conn)
            .await
            .unwrap()
    }

    pub async fn get_mate_by_matching_proxy_tag(
        &self,
        conn: PgConnection,
        content: String,
    ) -> Option<Mate> {
        let mates = self.get_mates(conn).await;

        // FIXME: short-circuiting?
        let mut filtered_mates = mates.into_iter().filter(|mate| {
            let proxy_tags = &mate.proxy_tags;

            proxy_tags.iter().any(|(prefix, suffix)| {
                let prefix = prefix.clone().unwrap_or("".to_string());
                let suffix = suffix.clone().unwrap_or("".to_string());

                content.contains(&format!("{}{}{}", prefix, mate.name, suffix))
            })
        });

        filtered_mates.next()
    }

    pub async fn insert_or_update(self, mut conn: PgConnection) {
        sqlx::query(
            "INSERT INTO collectives (id, user_id, name, description, pronouns, banner_url, \
             thumbnail_url, collective_tag) VALUES (?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT (id) DO \
             UPDATE SET user_id = EXCLUDED.user_id, name = EXCLUDED.name, description = \
             EXCLUDED.description, pronouns = EXCLUDED.pronouns, banner_url = \
             EXCLUDED.banner_url, thumbnail_url = EXCLUDED.thumbnail_url, collective_tag = \
             EXCLUDED.collective_tag",
        )
        .bind(self.id)
        .bind(self.user_id)
        .bind(self.name)
        .bind(self.description)
        .bind(self.pronouns)
        .bind(self.banner_url)
        .bind(self.thumbnail_url)
        .bind(self.collective_tag)
        .execute(&mut conn)
        .await
        .unwrap();
    }
}
