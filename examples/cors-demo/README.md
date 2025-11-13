# CORS Example

Demonstrates Cross-Origin Resource Sharing (CORS) middleware.

## Run

```bash
cargo run
```

## Test

```bash
# Test from browser console at http://localhost:8080
fetch('http://127.0.0.1:3040/api/users')
  .then(r => r.json())
  .then(console.log)
```
