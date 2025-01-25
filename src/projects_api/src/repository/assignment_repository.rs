use crate::model::prelude::*;
use chrono::Utc;
use sqlx::{sqlite::SqlitePool, Result, Row};

pub struct AssignmentRepository {
    pool: SqlitePool,
}

impl AssignmentRepository {
    pub fn new(pool: SqlitePool) -> Self {
        AssignmentRepository { pool }
    }

    pub async fn create_assignment(&self, assignment: &Assignment) -> Result<()> {
        sqlx::query(
            r#"
        INSERT INTO assignments (project_id, team_id, status, start_date, end_date, repository)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        )
        .bind(&assignment.project_id)
        .bind(&assignment.team_id)
        .bind(Status::Planned.to_string())
        .bind(&assignment.start_date.to_string())
        .bind(&assignment.end_date.to_string())
        .bind(&assignment.repository)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn change_assignment_status(
        &self,
        project_id: u32,
        team_id: u32,
        status: Status,
    ) -> Result<()> {
        sqlx::query(
            r#"
        UPDATE assignments SET status = ? WHERE project_id = ? AND team_id = ?
        "#,
        )
        .bind(project_id)
        .bind(team_id)
        .bind(status.to_string())
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    pub async fn get_assignment(&self, project_id: u32, team_id: u32) -> Result<Assignment> {
        let assignment_row = sqlx::query(
            r#"
            SELECT project_id, team_id, status, start_date, end_date, repository
            FROM assignments
            WHERE project_id = ? and team_id=?
            "#,
        )
        .bind(project_id)
        .bind(team_id)
        .fetch_one(&self.pool)
        .await?;

        let assignment = Assignment {
            project_id,
            team_id,
            status: Status::from(assignment_row.get::<&str, _>(2)),
            start_date: assignment_row
                .get::<String, _>(3)
                .parse()
                .unwrap_or(Utc::now()),
            end_date: assignment_row
                .get::<String, _>(4)
                .parse()
                .unwrap_or(Utc::now()),
            repository: assignment_row.get::<String, _>(5),
        };

        Ok(assignment)
    }
}
