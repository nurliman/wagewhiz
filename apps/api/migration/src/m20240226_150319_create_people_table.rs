use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS people (
                id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
                updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
                deleted_at TIMESTAMPTZ,
                name TEXT,
                nip TEXT,
                country TEXT,
                city TEXT,
                address TEXT,
                zip_code TEXT,
                email TEXT,
                phone TEXT,
                birthday DATE,
                organization TEXT,
                role TEXT,
                department TEXT,
                joining_date DATE,
                
                -- new fields
                is_active BOOLEAN DEFAULT TRUE,
                gender VARCHAR(20),
                status VARCHAR(20),
                avatar_url TEXT
            )",
        )
        .await?;

        db.execute_unprepared(
            "INSERT INTO people (name, email, department, role, joining_date, gender, nip, phone, status)
            VALUES
              ('John Doe', 'johndoe@example.com', 'Human Resources', 'HR Manager', '2022-01-01', 'Male', '1234567890', '+6281234567890', 'Permanent'),
              ('Jane Smith', 'janesmith@example.com', 'Sales', 'Sales Representative', '2022-02-01', 'Female', '0987654321', '+6289876543210', 'Contract'),
              ('Alex Johnson', 'alexjohnson@example.com', 'Marketing', 'Marketing Manager', '2021-12-01', 'Non-binary', '5678901234', '+6285678901234', 'Probation'),
              ('Emily Wilson', 'emilywilson@example.com', 'Finance', 'Financial Analyst', '2023-03-15', 'Female', '4321098765', '+6284321098765', 'Probation'),
              ('Michael Davis', 'michaeldavis@example.com', 'Engineering', 'Software Engineer', '2022-05-01', 'Male', '7890123456', '+6287890123456', 'Permanent'),
              ('Sarah Lee', 'sarahlee@example.com', 'Operations', 'Operations Manager', '2022-04-01', 'Female', '6543210987', '+6286543210987', 'Permanent'),
              ('David Brown', 'davidbrown@example.com', 'Human Resources', 'HR Specialist', '2022-03-01', 'Male', '8901234567', '+6288901234567', 'Contract'),
              ('Olivia Wilson', 'oliviawilson@example.com', 'Sales', 'Sales Manager', '2022-02-15', 'Female', '3210987654', '+6283210987654', 'Contract'),
              ('Daniel Johnson', 'danieljohnson@example.com', 'Marketing', 'Marketing Specialist', '2023-01-01', 'Male', '0123456789', '+6280123456789', 'Permanent'),
              ('Sophia Davis', 'sophiadavis@example.com', 'Finance', 'Accountant', '2022-11-01', 'Female', '8765432109', '+6288765432109', 'Contract'),
              ('James Wilson', 'jameswilson@example.com', 'Engineering', 'System Administrator', '2022-07-01', 'Male', '3456789012', '+6283456789012', 'Probation'),
              ('Emma Thompson', 'emmathompson@example.com', 'Operations', 'Operations Coordinator', '2022-06-01', 'Female', '6549873210', '+6286549873210', 'Permanent'),
              ('Andrew Brown', 'andrewbrown@example.com', 'Human Resources', 'Recruiter', '2022-09-01', 'Male', '8904561237', '+6288904561237', 'Contract'),
              ('Isabella Johnson', 'isabellajohnson@example.com', 'Sales', 'Sales Representative', '2022-08-01', 'Female', '3216540987', '+6283216540987', 'Probation'),
              ('Christopher Miller', 'christophermiller@example.com', 'Marketing', 'Marketing Coordinator', '2022-12-01', 'Male', '0129876543', '+6280129876543', 'Permanent'),
              ('Ava Clark', 'avaclark@example.com', 'Finance', 'Financial Manager', '2022-10-01', 'Female', '8765432109', '+6288765432109', 'Permanent'),
              ('Matthew Thompson', 'matthewthompson@example.com', 'Engineering', 'Software Engineer', '2023-05-01', 'Male', '3456789012', '+6283456789012', 'Permanent'),
              ('Grace Baker', 'gracebaker@example.com', 'Operations', 'Operations Manager', '2023-04-01', 'Female', '6549873210', '+6286549873210', 'Contract'),
              ('Benjamin Turner', 'benjaminturner@example.com', 'Human Resources', 'HR Specialist', '2023-03-01', 'Male', '8904561237', '+6288904561237', 'Probation'),
              ('Lily Moore', 'lilymoore@example.com', 'Sales', 'Sales Manager', '2023-02-01', 'Female', '3216540987', '+6283216540987', 'Permanent');",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE people")
            .await?;

        Ok(())
    }
}
