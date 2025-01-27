use crate::dto::prelude::{CreatedTeam, NewHistory};
use crate::model::prelude::*;
use crate::repository::history_repository::HistoryRepository;
use sqlx::{sqlite::SqlitePool, Result, Row};

pub struct TeamRepository {
    pool: SqlitePool,
    history_repository: HistoryRepository,
}

impl TeamRepository {
    pub fn new(pool: SqlitePool) -> Self {
        let history_repository = HistoryRepository::new(pool.clone());
        TeamRepository {
            pool,
            history_repository,
        }
    }

    pub async fn create_team(&self, team: &Team) -> Result<CreatedTeam> {
        let inserted: (i64,) = sqlx::query_as(
            r#"
            INSERT INTO teams (name)
            VALUES (?)
            RETURNING id
            "#,
        )
        .bind(&team.name)
        .fetch_one(&self.pool)
        .await?;

        log::info!("Auto created row id {}", inserted.0);

        for member in &team.members {
            self.add_member_to_team(inserted.0 as u32, member).await?;
        }

        Ok(CreatedTeam {
            id: inserted.0 as u32,
            name: team.name.clone(),
            member_count: team.members.len(),
        })
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

    pub async fn update_team_members_scores(&self, team_id: u32, score: u32) -> Result<u64> {
        let updated = sqlx::query(
            r#"
            UPDATE members SET score = ?
            WHERE team_id = ?
            "#,
        )
        .bind(score)
        .bind(team_id)
        .execute(&self.pool)
        .await?;

        if updated.rows_affected() > 0 {
            self.history_repository
                .create_history(&NewHistory {
                    event: "ScoresUpdated".to_string(),
                    description: format!(
                        "Team no '{}' members scores updated to '{}'",
                        team_id, score
                    ),
                })
                .await?;
        }

        Ok(updated.rows_affected())
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
            SELECT id, identity, full_name, score
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
                id: row.get::<u32, _>(0),
                identity: row.get::<String, _>(1),
                full_name: row.get::<String, _>(2),
                score: row.get::<i32, _>(3),
            })
            .collect();

        Ok(Team {
            id: team.id,
            name: team.name,
            members,
        })
    }
}
