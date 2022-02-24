use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use uuid::Uuid;

use crate::Database;

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy, strum::EnumString, strum::Display)]
pub enum InvitationMode {
    PasswordReset,
    JoinOrganization,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Invitation {
    uid: Uuid,
    target: Uuid,
    mode: InvitationMode,
}

impl Invitation {
    pub fn new(target: Uuid, mode: InvitationMode) -> Self {
        Invitation {
            uid: Uuid::new_v4(),
            target,
            mode,
        }
    }

    pub fn uid(&self) -> Uuid {
        self.uid
    }

    pub fn target(&self) -> Uuid {
        self.target
    }

    pub fn mode(&self) -> InvitationMode {
        self.mode
    }

    pub async fn load(db: &Database, uid: Uuid) -> Result<Option<Invitation>, sqlx::Error> {
        Ok(sqlx::query_as!(
            Invitation,
            r#"
SELECT "uid", "target", "mode" AS "mode: InvitationMode"
FROM "c_invitation"
WHERE "uid" = $1 LIMIT 1
"#,
            uid
        )
        .fetch_optional(db)
        .await?)
    }

    pub async fn store(&mut self, db: &Database) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
INSERT INTO "c_invitation" ("uid", "target", "mode")
VALUES ($1, $2, $3)
ON CONFLICT ("uid") DO
UPDATE SET "target" = $2, "mode" = $3
"#,
            self.uid,
            self.target,
            self.mode.to_string(),
        )
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn consume(self, db: &Database) -> Result<(), (Invitation, sqlx::Error)> {
        match sqlx::query!(
            r#"
DELETE FROM "c_invitation"
WHERE "uid" = $1
"#,
            self.uid
        )
        .execute(db)
        .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err((self, err)),
        }
    }
}
