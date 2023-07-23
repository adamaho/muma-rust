use clap::{Parser, ValueEnum};
use muma_db::Database;
use muma_db_migration::{Migrator, MigratorTrait};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Command {
    Up,
    Down,
    Fresh,
    Reset,
    Refresh,
    Status,
}

/// Applies migrations for a specific env.
#[derive(Parser, Debug)]
#[command(about = "Apply, rollback or get a status of all migrations.")]
struct Args {
    /// The migration command to run.  
    #[arg(short, long)]
    command: Command,

    /// The number of migrations to apply or rollback.
    #[arg(short, long)]
    amount: Option<u32>,

    /// The env file to use for applying migrations. Should contain an env var DATABASE_URL.
    #[arg(short, long)]
    env: Option<String>,
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let env = args.env.unwrap_or(String::from("../.env.development"));
    let db = Database::from_env_path(&env)?.connect().await?;

    match args.command {
        Command::Up => {
            println!("Running pending migrations...");
            Migrator::up(&db, args.amount).await?;
            println!("Migration complete.");
        }
        Command::Down => {
            println!("Rolling back last applied migrations...");
            Migrator::up(&db, args.amount).await?;
            println!("Rollback complete.");
        }
        Command::Status => {
            println!("Checking status of migrations...");
            Migrator::status(&db).await?;
        }
        Command::Fresh => {
            println!("Dropping database and applying migrations...");
            Migrator::fresh(&db).await?;
            println!("Fresh database created.");
        }
        Command::Refresh => {
            println!("Rolling back and reapplying migrations...");
            Migrator::refresh(&db).await?;
            println!("Rollback and reapply complete.");
        }
        Command::Reset => {
            println!("Rolling back all migrations...");
            Migrator::reset(&db).await?;
            println!("Rollback complete.");
        }
    }

    Ok(())
}
