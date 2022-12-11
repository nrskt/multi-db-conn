# Sandbox for using multiple database in Rust

## How to run

### 1. Setup diesel

See `diesel` official documentation and install client(diesel-cli).

Doc: https://diesel.rs/guides/getting-started.html

```shell
$ cargo install diesel_cli --no-default-features --features postgres
```

### 2. Start two PostgreSQL containers by docker-compose

```shell
docker compose up -d
```

### 3. Migrate the databases and insert sample data

```shell
$ diesel migration run --database-url postgres://postgres:password1@localhost:15432/docker
$ diesel migration run --database-url postgres://postgres:password2@localhost:15433/docker
```

```shell
$ psql postgres://postgres:password1@localhost:15432/docker -c "insert into sample_user values (1, 1, 'this is tenant_1')"
$ psql postgres://postgres:password2@localhost:15433/docker -c "insert into sample_user values (1, 2, 'this is tenant_2')"
```

### 4. Run sample code

```shell
$ cargo run
```

You can get the following results.

```
tenant_id: 1
Ok(
    [
        SampleUser {
            id: 1,
            tenant_id: 1,
            value: "this is tenant_1",
        },
    ],
)
tenant_id: 2
Ok(
    [
        SampleUser {
            id: 1,
            tenant_id: 2,
            value: "this is tenant_2",
        },
    ],
)
```
