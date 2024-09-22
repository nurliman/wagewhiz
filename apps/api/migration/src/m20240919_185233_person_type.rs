use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS person_types (
                id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
                updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
                deleted_at TIMESTAMPTZ,
                name TEXT,
                description TEXT
            );",
        )
        .await?;

        db.execute_unprepared(
            "INSERT INTO person_types (name, description)
            VALUES
                ('Employee', 'Full-time employee of the company'),
                ('Contractor', 'Temporary worker on contract'),
                ('Intern', 'Student or trainee working to gain experience'),
                ('Freelancer', 'Self-employed worker hired for specific projects'),
                ('Consultant', 'External expert providing professional advice');",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE person_types")
            .await?;

        Ok(())
    }
}
