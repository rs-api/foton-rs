# Extensions Example

Demonstrates request extensions for passing data between middleware and handlers.

## Run

```bash
cargo run
```

## Test

```bash
# Anonymous request
curl http://127.0.0.1:3007/

# Authenticated request
curl http://127.0.0.1:3007/ -H "Authorization: Bearer token123"

# Admin route (requires auth)
curl http://127.0.0.1:3007/admin
curl http://127.0.0.1:3007/admin -H "Authorization: Bearer token123"
```
