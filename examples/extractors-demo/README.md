# Extractors Example

Demonstrates all available request extractors.

## Run

```bash
cargo run
```

## Test

```bash
# Path parameters
curl http://127.0.0.1:3030/users/42

# Query parameters
curl "http://127.0.0.1:3030/search?q=rust&page=2"

# JSON body
curl -X POST http://127.0.0.1:3030/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice","email":"alice@example.com"}'

# Form data
curl -X POST http://127.0.0.1:3030/login \
  -d "username=admin&password=secret"

# Headers
curl http://127.0.0.1:3030/headers \
  -H "Authorization: Bearer token123"

# Raw bytes
curl -X POST http://127.0.0.1:3030/upload \
  -d "binary data here"

# Multiple extractors
curl -X POST http://127.0.0.1:3030/posts/10/comments \
  -H "Content-Type: application/json" \
  -d '{"name":"Bob","email":"bob@example.com"}'
```
