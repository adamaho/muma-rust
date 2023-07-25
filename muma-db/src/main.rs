use muma_db::db::Database;
use muma_db::query::restaurant;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let db = Database::from_env_path(".env.development")?
        .connect()
        .await?;

    println!("Create a new restaurant");
    restaurant::insert(
        &db,
        restaurant::InsertRestaurant {
            name: String::from("I love sweetie very much"),
            address_line1: None,
            address_line2: None,
            city: None,
            country: None,
            postal_code: None,
            state: None,
        },
    )
    .await?;

    println!("Select all of the restaurants");
    let foo = restaurant::select_by_id(&db, 1).await?;

    println!("restaurants: {:?}", foo);

    Ok(())
}
