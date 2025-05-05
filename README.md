# wallhaven-api-rs

>>> I swear this will soon get updated to a better code + docstrings
>>> Please contribute to this, I have as much free time as the next guy

A wallhaven api wrapper. I found just CLIs and such on cargo, so I might as well create one for my project

## Usage


### Creating the client
First, you must create a `WallhavenClient`:
```rust
let api_key = std::env::var("WALLHAVEN_API_KEY").ok();
let client = WallhavenClient::new(api_key.as_deref())?;
```

This will create a `WallhavenClient` that will either run with or without api key. You could as well just:
```rust
let client = WallhavenClient::new(None);
// Or
let client = WallhavenClient::new(Some("SomeApiKeyIDoNotSuggestHardcoding"));
```

### Using the client
