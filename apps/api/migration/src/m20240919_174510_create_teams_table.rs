use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS teams (
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
            "INSERT INTO teams (name, description)
            VALUES
                ('Engineering', 'Responsible for developing and maintaining software products and infrastructure.'),
                ('Marketing', 'Handles product promotion, brand management, and customer acquisition strategies.'),
                ('Sales', 'Focuses on client relationships, product sales, and revenue generation.'),
                ('Human Resources', 'Manages employee relations, recruitment, and overall workplace culture.'),
                ('Finance', 'Oversees financial planning, accounting, and economic strategy for the company.'),
                ('Customer Support', 'Provides assistance and technical support to customers using our products.'),
                ('Product Management', 'Guides product vision, roadmap, and feature development across the organization.'),
                ('Design', 'Creates user interfaces and experiences for our products and marketing materials.');",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE teams")
            .await?;

        Ok(())
    }
}
