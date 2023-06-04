# Blog Api - Rust

This is an API developed with Rust, Rocket & Diesel and was created in order to study Rust. Therefore, the code might not be one of the best since I'm still a noob :D.

## Executing

2 - Install `diesel` if you haven't yet:

```sh
cargo install diesel_cli --no-default-features --features postgres
```

3 - Run the container:

```sh
docker compose up
```

4 - Create the tables:

```sh
diesel migration run
```

5 - Run the app:

```sh
cargo run
```

## Postman

Posting a new content.

**POST:** `http://localhost:8000/post`

Body example:

```json
{
  "title": "My First Post",
  "body": "The awesome body of my first post! :D"
}
```

Fetching posts.

**GET:** `http://localhost:8000/posts`

## Good to know

If you face the `Id: library not found for -lpq`. Access [this site](https://stackoverflow.com/questions/70313347/ld-library-not-found-for-lpq-when-build-rust-in-macos) to get to know how to fix it.

### Tips

`diesel setup`: Initializes the migrations setup for diesel

`diesel migration generate posts`: Generates posts migration table

`diesel migration run`: Runs migration codes

`diesel migration redo`: It's going to execute the down.sql and up.sql file again in order to redo the migration.
