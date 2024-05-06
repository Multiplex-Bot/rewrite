use sqlx::{types::Uuid, FromRow};

#[derive(FromRow, Default)]
pub struct Mate {
    pub id: Uuid,
    pub collective_id: Uuid,
    pub name: String,
    pub display_name: String,
    pub pronouns: String,
    pub description: String,
    pub avatar_url: String,
    pub is_public: bool, // FIXME: Finegrained privacy settings?
    pub proxy_tags: Vec<(Option<String>, Option<String>)>,
    pub signature: (Option<String>, Option<String>),
}

impl Mate {
    pub async fn get_by_id(id: Uuid, mut conn: sqlx::PgConnection) -> Option<Mate> {
        sqlx::query_as("SELECT * FROM mates WHERE id = ?")
            .bind(id)
            .fetch_optional(&mut conn)
            .await
            .unwrap()
    }
}
