# 1. Create database and write sql code

1. Run in bash
```bash
sqlx migration run
```

2. This is users table
```sql
CREATE TABLE users (
    id bigserial PRIMARY KEY,
    name varchar(64) NOT NULL,
    email varchar(64) NOT NULL,
    -- hashed argon2 password, length 97
    password_hash varchar(97) NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);
```

# 2. Generating public ed25519 key with OpenSSL

1. JWT Ed25519
```bash
# Generate ed25519 privkey
openssl genpkey -algorithm ed25519 -out encoding.pem
# export its pubkey
openssl pkey -in encoding.pem -pubout -out decoding.pem
```

# 3. shortener
```sql
CREATE TABLE urls (
    id CHAR(6) PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id),
    url TEXT NOT NULL UNIQUE
)
```
