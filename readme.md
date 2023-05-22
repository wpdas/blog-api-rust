1 - Create a `blog` database using Postgres. This is using docker to create it.

## Commands

`diesel setup`: Initializes the migrations setup for diesel

`diesel migration generate posts`: Generates posts migration table

`diesel migration run`: Runs migration codes

`diesel migration redo`: It's going to execute the down.sql and up.sql file again in order to redo the migration.
