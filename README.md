- [Intro](#intro)
- [Schema](#schema)
- [Indices](#indices)
- [Views](#views)
- [Caching](#caching)
- [Optimisation Benchmarking](#optimisation-benchmarking)
- [Cache Testing](#cache-testing)

## Intro
This repo was my experimenting with SQL optimisation and code level caching.
A simple banking PostgreSQL database is used with Rust for programming.

## Schema
```mermaid
erDiagram
    users ||--o{ accounts : has
    users ||--o{ loans : takes
    users ||--|{ audit_logs : audits
    users {
        int id PK
        uuid public_id
        varchar given_name
        varchar family_name
        varchar username
        varchar email
        varchar phone
        timestamp created_at
    }

    accounts ||--o{ cards : has
    accounts ||--o{ transfers : sends
    accounts ||--o{ transfers : receives
    accounts ||--o{ transactions : makes
    accounts ||--o{ payments : makes
    accounts {
        int id PK
        int user_id FK
        varchar account_type
        decimal balance
        char currency
        timestamp created_at
    }

    cards {
        int id PK
        int account_id FK
        varchar card_number
        varchar card_type
        date expiration_date
        varchar status
    }

    transfers {
        int id PK
        int sender_account_id FK
        int receiver_account_id FK
        decimal amount
        char currency
        varchar status
        timestamp created_at
    }

    transactions {
        int id PK
        int account_id FK
        varchar type
        decimal amount
        char currency
        varchar status
        timestamp created_at
    }

    loans ||--o{ payments : has
    loans {
        int id PK
        int user_id FK
        decimal amount
        decimal interest_rate
        int term_months
        varchar status
        timestamp created_at
    }

    payments {
        int id PK
        int account_id FK
        int loan_id FK
        decimal amount
        char currency
        varchar status
        timestamp created_at
    }

    audit_logs {
        int id PK
        int user_id FK
        varchar action
        text details
        timestamp timestamp
    }
```
## Indices
## Views
## Caching
## Optimisation Benchmarking
## Cache Testing
