use muma_db_sqlx::Database;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let pool = Database::from_env_path("../.env.development")?
        .connect()
        .await?;

    println!("Migrating");
    sqlx::migrate!().run(&pool).await?;
    Ok(())
}
