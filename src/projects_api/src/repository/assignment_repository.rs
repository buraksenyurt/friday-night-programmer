use crate::dto::prelude::NewHistory;
use crate::model::prelude::*;
use crate::repository::history_repository::HistoryRepository;
use crate::utility::*;
use sqlx::{sqlite::SqlitePool, Result, Row};

pub struct AssignmentRepository {
    pool: SqlitePool,
    history_repository: HistoryRepository,
}

impl AssignmentRepository {
    pub fn new(pool: SqlitePool) -> Self {
        let history_repository = HistoryRepository::new(pool.clone());
        AssignmentRepository {
            pool,
            history_repository,
        }
    }

    pub async fn create_assignment(&self, assignment: &Assignment) -> Result<u64> {
        let inserted = sqlx::query(
            r#"
        INSERT INTO assignments (project_id, team_id, status, start_date, end_date, repository)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        )
        .bind(assignment.project_id)
        .bind(assignment.team_id)
        .bind(assignment.status.to_string())
        .bind(format_datetime(&assignment.start_date))
        .bind(format_datetime(&assignment.end_date))
        .bind(&assignment.repository)
        .execute(&self.pool)
        .await?;

        self.history_repository
            .create_history(&NewHistory {
                event: "CreatedNewAssignment".to_string(),
                description: format!("New assignment created for team {}", assignment.team_id),
            })
            .await?;

        Ok(inserted.rows_affected())
    }

    pub async fn change_assignment_status(
        &self,
        project_id: u32,
        team_id: u32,
        status: Status,
    ) -> Result<u64> {
        let old_status: (String,) = sqlx::query_as(
            r#"
        SELECT status FROM assignments WHERE project_id = ? AND team_id = ?
        "#,
        )
        .bind(project_id)
        .bind(team_id)
        .fetch_one(&self.pool)
        .await?;

        let updated = sqlx::query(
            r#"
        UPDATE assignments SET status = ? WHERE project_id = ? AND team_id = ?
        "#,
        )
        .bind(status.to_string())
        .bind(project_id)
        .bind(team_id)
        .execute(&self.pool)
        .await?;

        self.history_repository
            .create_history(&NewHistory {
                event: "AssignmentStatusChanged".to_string(),
                description: format!(
                    "Assignment status changed to {} from {}",
                    status.to_string(),
                    old_status.0
                ),
            })
            .await?;

        Ok(updated.rows_affected())
    }

    pub async fn get_assignment(&self, project_id: u32, team_id: u32) -> Result<Assignment> {
        let assignment_row = sqlx::query(
            r#"
        SELECT project_id, team_id, status, start_date, end_date, repository
        FROM assignments
        WHERE project_id = ? AND team_id = ?
        "#,
        )
        .bind(project_id)
        .bind(team_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Assignment {
            project_id: assignment_row.get::<u32, _>("project_id"),
            team_id: assignment_row.get::<u32, _>("team_id"),
            status: assignment_row
                .get::<&str, _>("status")
                .parse()
                .unwrap_or(Status::Planned),
            start_date: parse_datetime(&assignment_row.get::<String, _>("start_date")),
            end_date: parse_datetime(&assignment_row.get::<String, _>("end_date")),
            repository: assignment_row.get::<String, _>("repository"),
        })
    }
}
