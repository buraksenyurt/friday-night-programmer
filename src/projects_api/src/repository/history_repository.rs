use crate::dto::prelude::*;
use crate::model::prelude::History;
use crate::utility;
use chrono::Utc;
use sqlx::{sqlite::SqlitePool, Result, Row};

pub struct HistoryRepository {
    pool: SqlitePool,
}

impl HistoryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        HistoryRepository { pool }
    }

    pub async fn create_history(&self, history: &NewHistory) -> Result<()> {
        sqlx::query(
            r#"
        INSERT INTO history (time, event, description)
        VALUES (?, ?, ?)
        RETURNING id
        "#,
        )
        .bind(utility::format_datetime(&Utc::now()))
        .bind(&history.event)
        .bind(&history.description)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_history(&self) -> Result<Vec<History>> {
        let rows = sqlx::query(
            r#"
            SELECT time, event, description
            FROM history
            ORDER BY time DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let result = rows
            .iter()
            .map(|row| History {
                time: utility::parse_datetime(&row.get::<String, _>(0)),
                event: row.get::<String, _>(1),
                description: row.get::<String, _>(2),
            })
            .collect();

        Ok(result)
    }
}
