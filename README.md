## Contents

- [Intro](#intro)
- [Schema](#schema)
- [Testing](#testing)

## Intro
This repo was my experimenting with SQL optimisation and code level caching.
A simple banking PostgreSQL database is used with Rust for programming.

## Schema
```mermaid
erDiagram
    users ||--o{ accounts : has
    users ||--o{ loans : takes
    users ||--|{ audit_logs : has
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
    accounts ||--|{ audit_logs : has
    accounts {
        int id PK
        int user_id FK "Idx"
        varchar account_type
        decimal balance "Idx"
        char currency
        timestamp created_at
        int num_active_cards "Denorm"
    }

    cards ||--|{ audit_logs : has
    cards {
        int id PK
        int account_id FK
        varchar card_number
        varchar card_type
        date expiration_date
        varchar status
    }

    transfers ||--|{ audit_logs : has
    transfers {
        int id PK
        int sender_account_id FK
        int receiver_account_id FK
        decimal amount
        char currency
        varchar status
        timestamp created_at
    }

    transactions ||--|{ audit_logs : has
    transactions {
        int id PK
        int account_id FK "Idx"
        varchar transaction_type
        decimal amount "Idx"
        char currency
        varchar status
        timestamp created_at
    }

    loans ||--|{ audit_logs : has
    loans ||--o{ payments : has
    loans {
        int id PK
        int user_id FK "Idx"
        decimal amount "Idx"
        decimal interest_rate
        int term_months
        varchar status
        timestamp created_at
    }

    payments ||--|{ audit_logs : has
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
        int subject_table
        int subject_id
        varchar action
        text details
        timestamp created_at
    }
    
    mat_view_loans_outstanding |o--o| users : "aggregates"
    mat_view_loans_outstanding {
        int user_id
        int sum_loans_outstanding
    }

    mat_view_average_transaction_amount |o--o| accounts : "calculates"
    mat_view_average_transaction_amount {
        int account_id
        int average_transaction
    }

    mat_view_suspicious_transactions |o--o| transactions : "includes"
    mat_view_suspicious_transactions |o--o| accounts : "includes"
    mat_view_suspicious_transactions |o--o| mat_view_average_transaction_amount : "compares"
    mat_view_suspicious_transactions {
        int user_id
        int account_id
        int transaction_id
        int amount
        text risk_level
    }
```

## Testing
The SQLx cargo package testing functionality is used to test schema data insertion, materialized view vs raw query timings, and some sample queries. Caching tests are also run.
To run:
```sh
docker run --rm -d --name test-postgres -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=password -e POSTGRES_DB=testdb -p 5432:5432 postgres:latest
```
```sh
DATABASE_URL="postgres://postgres:password@localhost:5432/testdb" cargo nextest run --nocapture
```
```sh
docker stop test-postgres
```
