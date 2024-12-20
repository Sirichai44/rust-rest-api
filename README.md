# rust-rest-api
RESTful API with Rust
  - database: mongoDB

## Structure
```
📂rust_api
|   📄.gitignore
|   📄Cargo.toml
|   📄README.md 
|   📄Setting.toml
|   📄docker-compose.yml
| 
+---📂src/
   |    📄lib.rs 
   |    📄main.rs
   |    📄setting.rs
   |    📄database.rs 
   |    📄time_helper.rs 
   |
   +---📂entities/
   |    📄users.rs
   |    📄mod.rs
   |    
   +---📂handlers/
   |    📄users.rs
   |    📄mod.rs
   |    
   +---📂models/
   |    📄users.rs
   |    📄error.rs
   |    📄mod.rs
   |    
   +---📂repositories/
   |    📄users.rs
   |    📄mod.rs
   | 
   +---📂usecases/
        📄users_test.rs
        📄users.rs
        📄mod.rs

```
## How to Start
#### start database
   
```
docker-compse up
```
```
cargo run
```

**Serve on post 8080**
