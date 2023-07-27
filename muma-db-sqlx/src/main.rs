use muma_db_sqlx::user;
use muma_db_sqlx::Database;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let pool = Database::from_env_path(".env")?.connect().await?;
    sqlx::migrate!().run(&pool).await?;

    let user_id = sqlx::query!(
        r#"
INSERT INTO users ( username )
VALUES ( "i love sweetieeeeeeee" )
        "#,
    )
    .execute(&pool)
    .await?
    .last_insert_id()
    .try_into()?;

    let user = user::select_by_id(&pool, user_id).await?;

    println!("{:?}", user);
    Ok(())
}
