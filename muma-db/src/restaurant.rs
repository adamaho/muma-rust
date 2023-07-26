use sea_orm::entity::prelude::*;
use sea_orm::IntoActiveModel;
use sea_orm::Set;
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

#[derive(Debug, Deserialize, DeriveIntoActiveModel)]
pub struct UpdateRestaurant {
    pub name: String,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>,
}

pub async fn update(
    conn: &impl ConnectionTrait,
    restaurant_id: i32,
    restaurant: UpdateRestaurant,
) -> anyhow::Result<Model, DbErr> {
    let r: ActiveModel = Entity::find_by_id(restaurant_id)
        .one(conn)
        .await?
        .ok_or(DbErr::Custom(format!(
            "Could not find restaurant with id: {}",
            restaurant_id.clone(),
        )))
        .map(Into::into)?;

    ActiveModel {
        restaurant_id: r.restaurant_id,
        name: Set(restaurant.name),
        address_line1: Set(restaurant.address_line1),
        address_line2: Set(restaurant.address_line2),
        city: Set(restaurant.city),
        country: Set(restaurant.country),
        postal_code: Set(restaurant.postal_code),
        state: Set(restaurant.state),
        created_at: r.created_at,
    }
    .update(conn)
    .await
}
