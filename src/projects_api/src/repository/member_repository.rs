use crate::dto::prelude::NewHistory;
use crate::enums::history_event::HistoryEvent;
use crate::repository::history_repository::HistoryRepository;
use sqlx::SqlitePool;

pub struct MemberRepository {
    pool: SqlitePool,
    history_repository: HistoryRepository,
}

impl MemberRepository {
    pub fn new(pool: SqlitePool) -> MemberRepository {
        let history_repository = HistoryRepository::new(pool.clone());
        MemberRepository {
            pool,
            history_repository,
        }
    }

    pub async fn move_member_to_another_team(
        &self,
        member_id: u32,
        team_id: u32,
    ) -> sqlx::Result<u64> {
        let updated = sqlx::query(
            r#"
            UPDATE members SET team_id = ?
            WHERE id = ?
            "#,
        )
        .bind(team_id)
        .bind(member_id)
        .execute(&self.pool)
        .await?;

        if updated.rows_affected() > 0 {
            self.history_repository
                .create_history(&NewHistory {
                    event: HistoryEvent::MemberMoved.to_string(),
                    description: format!("Member no '{}' moved to team '{}'", member_id, team_id),
                })
                .await?;
        }

        Ok(updated.rows_affected())
    }
}
