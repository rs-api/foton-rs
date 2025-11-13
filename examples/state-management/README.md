# State Management Example

Demonstrates shared state across handlers using atomic counters.

## Run

```bash
cargo run
```

## Test

```bash
# Get current count
curl http://127.0.0.1:3001/count

# Increment counter
curl -X POST http://127.0.0.1:3001/increment

# Check count again
curl http://127.0.0.1:3001/count
```
