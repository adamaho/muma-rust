use sea_orm::entity::prelude::*;
use sea_orm::IntoActiveModel;
use serde::Deserialize;

use crate::model::restaurant::{ActiveModel, Entity as RestaurantEntity, Model as RestaurantModel};

/// Selects a restaurant by a the provided restaurant id
///
/// Example:
/// ```
/// let restaurant = restaurant::select_by_id(&db, 1).await?;
/// ```
pub async fn select_by_id(
    db: &impl ConnectionTrait,
    id: i32,
) -> anyhow::Result<Option<RestaurantModel>> {
    let restaurant: Option<RestaurantModel> = RestaurantEntity::find_by_id(id).one(db).await?;
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
) -> anyhow::Result<RestaurantModel, DbErr> {
    new_restaurant.into_active_model().insert(conn).await
}
