# muma-db-sqlx

Handles the database integration with PlanetScale.

## Install

Install the sqlx-cli by running:

```bash
cargo install sqlx-cli --features rustls
```

## Environment

Configure your `.env` to include the following environment variables:

```bash
DATBASE_URL=""
```

## Working with PlanetScale

The current choice of database is PlanetScale which is built on top of mysql.

## Install

Make sure to install the PlanetScale [cli]("https://planetscale.com/features/cli"). Once installed you can run the following command to login.

```bash
pscale auth login
```

### Dropping and Recreating the Database

Sometimes it is nice to start from a fresh database. Make sure you have the proper branch checked out and run the following command:

```bash
sqlx migrate revert
```

Then recreate the database:

```bash
sqlx migrate run
```

## Migrations

Make sure the sqlx-cli is installed. See Install for more details.

### Creating a new Migration

Create a new migration:

```bash
sqlx migrate add -r <migration_name>
```

### Running migrations

Run all pending migrations:

```bash
sqlx migrate run
```

Revert any previously run migrations:

```bash
sqlx migrate revert
```

### Prepare Queries

Running `prepare` creates a directory `.sqlx` that the compiler uses to validate queries to the database
and ensure they are fully typesafe. Useful for working offline and running tests in CI.

```bash
cargo sqlx prepare
```

## Seeding the database

tbd.


