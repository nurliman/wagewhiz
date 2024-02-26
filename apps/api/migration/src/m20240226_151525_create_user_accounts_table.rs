use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS user_accounts (
                id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
                updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
                deleted_at TIMESTAMPTZ,
                username VARCHAR NOT NULL,
                password TEXT NOT NULL,
                role VARCHAR NOT NULL,
                person_id uuid,
                FOREIGN KEY (person_id) REFERENCES people(id)
            );",
        )
        .await?;

        // add admin user
        db.execute_unprepared("
            INSERT INTO user_accounts (username, password, role)
            VALUES ('admin', '$argon2i$v=19$m=16,t=2,p=1$cTNiYkxtSFZwUXVObGdqQw$w2Jx6n8dZgEh9eXiCBoinw', 'admin');
        ")
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE `user_accounts`")
            .await?;

        Ok(())
    }
}
