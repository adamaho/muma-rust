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