CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    public_id UUID UNIQUE DEFAULT gen_random_uuid(),
    given_name VARCHAR(50) NOT NULL,
    family_name VARCHAR(50) NOT NULL,
    username VARCHAR(50) NOT NULL, -- Not UNIQUE due to low cardinality in use fake::faker::internet::en::Username
    email VARCHAR(100) NOT NULL, -- Not UNIQUE due to low cardinality in use fake::faker::internet::en::SafeEmail
    phone VARCHAR(50) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
