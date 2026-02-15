# codegang-datasource

A structured data source for AI agents working with microservice architectures. Stores service definitions, queue contracts, NoSQL entity contracts, and protobuf contracts — exposed through a REST API that AI agents can query to understand and operate on your infrastructure.

## Data Model

### Datasource

Top-level object containing all registries:

```json
{
  "services": [],
  "queue_contracts": [],
  "nosql_contracts": [],
  "proto_contracts": []
}
```

### Service

```json
{
  "name": "contest-engine-grpc",
  "type": "microservice",
  "github_repo": "https://github.com/org/contest-engine-grpc",
  "description": "Core contest orchestration engine.",
  "grpc_servers": ["ContestEngineGrpcService"],
  "grpc_clients": ["ClientWalletsGrpcService", "UsersGrpcService"],
  "queue": {
    "publish_queues": ["contest-registration-update"],
    "subscribe_queues": ["contest-accounts-updates"]
  },
  "is_http_server": false,
  "metadata": { "team": "trading" }
}
```

### Queue Contract

```json
{
  "topic_name": "user-registered",
  "description": "Emitted when a new user completes registration.",
  "message_schema": {
    "name": "UserRegisteredEvent",
    "fields": [
      { "name": "user_id", "field_type": "uuid", "description": "Unique user ID." },
      { "name": "email", "field_type": "string" },
      { "name": "registered_at", "field_type": "datetime" }
    ],
    "notes": "First event in user lifecycle."
  }
}
```

### NoSQL Contract

```json
{
  "entity_name": "BrokerContestSettings",
  "table_name": "broker-contest-settings",
  "description": "Per-broker contest configuration.",
  "schema": {
    "name": "BrokerContestSettings",
    "fields": [
      { "name": "broker_id", "field_type": "string" },
      { "name": "difficulty_level", "field_type": "string" },
      { "name": "max_leverage", "field_type": "i32" }
    ]
  }
}
```

### Proto Contract

```json
{
  "name": "UsersGrpcService",
  "raw_proto": "syntax = \"proto3\";\npackage users;\n\nservice UsersGrpcService {\n  rpc GetUserById (GetUserByIdRequest) returns (UserModel);\n}\n..."
}
```

## API

All mutations use **POST** with insert-or-replace semantics (upsert).

### Full Datasource

| Method | Endpoint           | Description                    |
|--------|--------------------|--------------------------------|
| `GET`  | `/api/datasource`  | Get entire datasource          |
| `PUT`  | `/api/datasource`  | Replace entire datasource      |

### Services

| Method   | Endpoint                | Description              |
|----------|-------------------------|--------------------------|
| `GET`    | `/api/services`         | List all services        |
| `POST`   | `/api/services`         | Insert or replace        |
| `GET`    | `/api/services/{name}`  | Get by name              |
| `DELETE` | `/api/services/{name}`  | Delete by name           |

### Queue Contracts

| Method   | Endpoint                         | Description              |
|----------|----------------------------------|--------------------------|
| `GET`    | `/api/queue-contracts`           | List all                 |
| `POST`   | `/api/queue-contracts`           | Insert or replace        |
| `GET`    | `/api/queue-contracts/{topic}`   | Get by topic name        |
| `DELETE` | `/api/queue-contracts/{topic}`   | Delete by topic name     |

### NoSQL Contracts

| Method   | Endpoint                          | Description              |
|----------|-----------------------------------|--------------------------|
| `GET`    | `/api/nosql-contracts`            | List all                 |
| `POST`   | `/api/nosql-contracts`            | Insert or replace        |
| `GET`    | `/api/nosql-contracts/{entity}`   | Get by entity name       |
| `DELETE` | `/api/nosql-contracts/{entity}`   | Delete by entity name    |

### Proto Contracts

| Method   | Endpoint                        | Description              |
|----------|---------------------------------|--------------------------|
| `GET`    | `/api/proto-contracts`          | List all                 |
| `POST`   | `/api/proto-contracts`          | Insert or replace        |
| `GET`    | `/api/proto-contracts/{name}`   | Get by name              |
| `DELETE` | `/api/proto-contracts/{name}`   | Delete by name           |

## Running

```bash
cargo run
```

Server starts on `http://0.0.0.0:8080`. Data persists to a local JSON file.

### Environment Variables

| Variable    | Default                    | Description                     |
|-------------|----------------------------|---------------------------------|
| `DATA_FILE` | `codegang-datasource.json` | Path to the JSON persistence file |

### Docker

```bash
docker run -p 8080:8080 -v $(pwd)/data:/data -e DATA_FILE=/data/codegang-datasource.json ghcr.io/hays-codegang/codegang-datasource:latest
```

## Project Structure

```
src/
  main.rs              # Routes and server setup
  model.rs             # Data model structs
  storage.rs           # In-memory state + JSON file persistence
  handlers/
    mod.rs             # Module declarations
    datasource.rs      # GET/PUT full datasource
    services.rs        # Service CRUD
    queue.rs           # Queue contract CRUD
    nosql.rs           # NoSQL contract CRUD
    proto.rs           # Proto contract CRUD
```

## Tech Stack

- **Language:** Rust
- **Web framework:** Actix-web 4
- **Persistence:** JSON file on disk
- **Container:** Debian Bookworm slim
- **CI/CD:** GitHub Actions — builds on push/PR, publishes to GHCR on release
