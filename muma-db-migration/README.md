# muma-db-migration 

Handles running and creating database migrations. 

## Creating a new migration

Creates a new migration in the `src` dir. Run from the root of the project.

    ```bash
    sea-orm-cli migrate generate -d muma-db-migration <name-of-migration> 
    ```

## Working with muma migration cli

Run the following command to get a list of options:

    ```bash
    cargo run -- -h
    ```

Or to get going quickly, run the following command to apply all migrations

    ```bash
    cargo run -- -c up -e ../.env.development
    ```