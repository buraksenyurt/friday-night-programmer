use crate::model::prelude::*;
use sqlx::{sqlite::SqlitePool, Result, Row};

pub struct TeamRepository {
    pool: SqlitePool,
}

impl TeamRepository {
    pub fn new(pool: SqlitePool) -> Self {
        TeamRepository { pool }
    }

    pub async fn create_team(&self, team: &Team) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO teams (id, name)
            VALUES (?, ?)
            "#,
        )
        .bind(team.id)
        .bind(&team.name)
        .execute(&self.pool)
        .await?;

        for member in &team.members {
            self.add_member_to_team(team.id, member).await?;
        }

        Ok(())
    }

    pub async fn add_member_to_team(&self, team_id: u32, member: &Member) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO members (identity, full_name, score, team_id)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(&member.identity)
        .bind(&member.full_name)
        .bind(member.score)
        .bind(team_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_team(&self, team_id: u32) -> Result<Team> {
        let team_row = sqlx::query(
            r#"
            SELECT id, name
            FROM teams
            WHERE id = ?
            "#,
        )
        .bind(team_id)
        .fetch_one(&self.pool)
        .await?;

        let team = Team {
            id: team_row.get::<i32, _>(0) as u32,
            name: team_row.get::<String, _>(1),
            members: vec![],
        };

        let members_rows = sqlx::query(
            r#"
            SELECT identity, full_name, score
            FROM members
            WHERE team_id = ?
            "#,
        )
        .bind(team_id)
        .fetch_all(&self.pool)
        .await?;

        let members = members_rows
            .iter()
            .map(|row| Member {
                identity: row.get::<String, _>(0),
                full_name: row.get::<String, _>(1),
                score: row.get::<i32, _>(2),
            })
            .collect();

        Ok(Team {
            id: team.id,
            name: team.name,
            members,
        })
    }
}
