pub use sea_orm_migration::prelude::*;

mod m20240226_150319_create_people_table;
mod m20240226_151525_create_user_accounts_table;
mod m20240919_174510_create_teams_table;
mod m20240919_185233_person_type;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240226_150319_create_people_table::Migration),
            Box::new(m20240226_151525_create_user_accounts_table::Migration),
            Box::new(m20240919_174510_create_teams_table::Migration),
            Box::new(m20240919_185233_person_type::Migration),
        ]
    }
}
