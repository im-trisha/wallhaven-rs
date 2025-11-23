# wallhaven-rs

A wallhaven api wrapper. I found just CLIs and such on cargo, so I might as well create one for my project

This crate was for a short period known as `wallhaven-api`, I then did a complete overhaul and now the code is actually nice!

## Usage

### Creating the client
First, you must create a `WallhavenClient`:
```rust
let api_key = std::env::var("WALLHAVEN_API_KEY").ok();
let client = WallhavenClient::new(api_key)?;
```

This will create a `WallhavenClient` that will either run with or without api key. You could as well just:
```rust
let client = WallhavenClient::new();
// Or
let client = WallhavenClient::with_key("SomeApiKeyIDoNotSuggestHardcoding");
```

### Using the client

The client has the following functions, and for more details you may check out the documentation of that function/returned model:

- `wallpaper(id: impl AsRef<str>)` - Fetches a wallpaper by id
- `search(params: Option<SearchRequest>)` - Searches for wallpapers, using the specific `SearchRequest`
- `collections(username: Option<String>)` - Searches for the collections of either the current user if the api key is provided (`None`), or some user (`Some("username")`)
- `collection_items(username: impl AsRef<str>, id: u64, params: Option<CollectionItemsRequest>)` - Gets the collection items for a certain user/collection, with optional search query
- `tag(id: u64)` - Fetches a tag's details
- `user_settings(&self)` - Fetches the current user's settings
- `download_wallpaper(&WallpaperDetails|&Wallpaper)` - Gets a stream of bytes to download that wallpaper
- `download_thumbnail(thumbnail: &Thumbnails, resolution: ThumbnailResolution)` - Gets a stream of bytes to download a thumbnail with a certain resolution
