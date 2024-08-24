## cargo watch

```
cargo watch -c -w src -x run
```

## seaorm

```
sea-orm-cli migrate fresh
sea-orm-cli generate entity -o entity/src
```

## Error Response

```
{
    "status": "Bad Request",
    "error_code": "INVALID_REQUEST",
    "error_info": {
        "category": "USER_ERROR",
        "developer_message": "Invalid input data",
        "request_id": "2d9257f8-29ec-43e6-90c6-25b586cfaaf8",
        "user_message": "Invalid input data"
    }
}
```

## References

axum + seaorm - https://github.com/trasherr/Blogging-API
tracing - https://www.shuttle.rs/blog/2024/01/09/getting-started-tracing-rust
custom error response for missing field in req - https://github.com/tokio-rs/axum/tree/a7d89541786167e990d095a03ac7bdba68d7a55a/examples/customize-extractor-error
