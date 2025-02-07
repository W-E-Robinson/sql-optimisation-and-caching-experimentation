erDiagram
    users {
        int id PK
        uuid public_id
        varchar given_name
        varchar family_name
        varchar username
        varchar password
        varchar email
        varchar phone
        timestamp created_at
    }

    accounts {
        int id PK
        int user_id FK
        varchar account_type
        decimal balance
        char currency
        timestamp created_at
    }
