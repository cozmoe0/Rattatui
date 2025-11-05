use super::Repository;
use crate::{entities::Agent, Error};
use log::error;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

impl Repository {
    pub async fn create_agent(&self, db: &Pool<Postgres>, agent: &Agent) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO agents
            (id, created_at, last_seen_at, identity_public_key, public_prekey, public_prekey_signature)
            VALUES ($1, $2, $3, $4, $5, $6)";

        sqlx::query(QUERY)
            .bind(agent.id)
            .bind(agent.created_at)
            .bind(agent.last_seen_at)
            .bind(&agent.identity_public_key)
            .bind(&agent.public_prekey)
            .bind(&agent.public_prekey_signature)
            .execute(db)
            .await
            .map_err(|err| {
                error!("create_agent: Inserting agent: {}", &err);
                err.into()
            })?;
        Ok(())
    }

    pub async fn update_agent(&self, db: &Pool<Postgres>, agent: &Agent) -> Result<(), Error> {
        const QUERY: &str = "UPDATE agents
            SET last_seen_at = $1
            WHERE id = $2";

        sqlx::query(QUERY)
            .bind(agent.last_seen_at)
            .bind(agent.id)
            .execute(db)
            .await
            .map_err(|err| {
                error!("update_agent: updating agent: {}", &err);
                err.into()
            })?;
        Ok(())
    }

    pub async fn find_all_agents(&self, db: &Pool<Postgres>) -> Result<Vec<Agent>, Error> {
        const QUERY: &str = "SELECT * FROM agents ORDER BY created_at";

        sqlx::query_as::<_, Agent>(QUERY)
            .fetch_all(db)
            .await
            .map_err(|err| {
                error!("find_all_agents: finding agents: {}", &err);
                err.into()
            })
    }

    pub async fn find_agent_by_id(
        &self,
        db: &Pool<Postgres>,
        agent_id: Uuid,
    ) -> Result<Agent, Error> {
        const QUERY: &str = "SELECT * FROM agents WHERE id = $1";

        sqlx::query_as::<_, Agent>(QUERY)
            .bind(agent_id)
            .fetch_optional(db)
            .await
            .map_err(|err| {
                error!("find_agent_by_id: finding agent: {}", &err);
                err.into()
            })?
            .ok_or_else(|| Error::NotFound("Agent not found.".into()))
    }
}
