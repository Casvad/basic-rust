
Install diesel ORM CLI
```shell
cargo install diesel_cli --no-default-features --features postgres
```

Init diesel
```shell
diesel setup
diesel migration generate create_post
```

to run migration 
```shell
diesel migration run
```

to revert migration
```shell
diesel migration redo
```

to run application
```shell
cargo run
```