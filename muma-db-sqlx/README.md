# muma-db-sqlx

Handles the database integration with PlanetScale.

## Install

Install the sqlx-cli by running:

    ```bash
    cargo install sqlx-cli --features rustls
    ```

## Migrations

Make sure the sqlx-cli is installed. See Install for more details.

### Creating a new Migration

Create a new migration by running the following command:

    ```bash
    sqlx migrate add <migration_name>
    ```

### Prepare Queries

Running `prepare` creates a directory `.sqlx` that the compiler uses to validate queries to the database
and ensure they are fully typesafe. Useful for working offline and running tests in CI.

    ```bash
    cargo sqlx prepare
    ```