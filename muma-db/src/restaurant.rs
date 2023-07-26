use sea_orm::entity::prelude::*;
use sea_orm::IntoActiveModel;
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

/// Selects a restaurant by a the provided restaurant id
///
/// Example:
/// ```
/// let restaurant = restaurant::select_by_id(&db, 1).await?;
/// ```
pub async fn select_by_id(db: &impl ConnectionTrait, id: i32) -> anyhow::Result<Option<Model>> {
    let restaurant: Option<Model> = Entity::find_by_id(id).one(db).await?;
    Ok(restaurant)
}

#[derive(Debug, Deserialize, DeriveIntoActiveModel)]
pub struct InsertRestaurant {
    pub name: String,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>,
}

/// Inserts a new restaurant into the db
///
/// Example:
/// ```
/// let new_restaurant = restaurant::insert(&db,
///     restaurant::InsertRestaurant {
///         name: String::from("Goho Pizza Co."),
///         address_line1: None,
///         address_line2: None
///         city: None,
///         country: None,
///         postal_code: None,
///         state: None
///     }
/// ).await?;
/// ```
pub async fn insert(
    conn: &impl ConnectionTrait,
    new_restaurant: InsertRestaurant,
) -> anyhow::Result<Model, DbErr> {
    new_restaurant.into_active_model().insert(conn).await
}
