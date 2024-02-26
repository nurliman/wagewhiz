use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_accounts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: TimeDateTimeWithTimeZone,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: TimeDateTimeWithTimeZone,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<TimeDateTimeWithTimeZone>,
    pub username: String,
    #[sea_orm(column_type = "Text")]
    #[serde(skip_serializing)]
    pub password: String,
    pub role: String,
    pub person_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::people::Entity",
        from = "Column::PersonId",
        to = "super::people::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    People,
}

impl Related<super::people::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::People.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
