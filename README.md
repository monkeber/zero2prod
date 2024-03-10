# ZeroToProd

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Postgres](https://img.shields.io/badge/postgres-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white)

This project was created while following the book 'Zero To Production In Rust' by Luca Palmieri.

## Tools

The following tools were used during development:
- Rust, v. 1.68.2
- PostgreSQL, v. 13

## Updating Sqlx Data

Run `cargo sqlx prepare` in the project directory.

## Build And Run

In order to setup necessary dependencies run the scripts from the project root:

```bash
./scripts/init_db.sh
./scripts/init_redis.sh
```

Then you can:

```bash
cargo test
```

## Default User

The default admin to use:

```
Login: admin
Password: everythinghastostartsomewhere
```
