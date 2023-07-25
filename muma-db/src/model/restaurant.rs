use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

//////////////////////////////////////////////////////////////////
/// Model                                                     
//////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "restaurants")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub restaurant_id: i32,
    pub name: String,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
