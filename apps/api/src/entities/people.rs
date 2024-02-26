use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "people")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: TimeDateTimeWithTimeZone,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: TimeDateTimeWithTimeZone,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<TimeDateTimeWithTimeZone>,
    #[sea_orm(column_type = "Text", nullable)]
    pub name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub nip: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub country: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub city: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub address: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub zip_code: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub email: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub phone: Option<String>,
    pub birthday: Option<Date>,
    #[sea_orm(column_type = "Text", nullable)]
    pub organization: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub role: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub department: Option<String>,
    pub joining_date: Option<Date>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_accounts::Entity")]
    UserAccounts,
}

impl Related<super::user_accounts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserAccounts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
