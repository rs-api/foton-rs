# Per-Route Middleware Example

Demonstrates applying middleware to specific routes vs globally.

## Run

```bash
cargo run
```

## Test

```bash
# Public route (only global middleware)
curl http://127.0.0.1:3008/

# Admin route (requires auth)
curl http://127.0.0.1:3008/admin
curl http://127.0.0.1:3008/admin -H "Authorization: Bearer token123"

# API route (requires JSON content-type)
curl -X POST http://127.0.0.1:3008/api/data
curl -X POST http://127.0.0.1:3008/api/data -H "Content-Type: application/json"
```
