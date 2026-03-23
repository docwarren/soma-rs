# soma_rocket

REST API server using the Rocket framework. Serves as the HTTP backend for the web app.

## Structure

- `src/main.rs` — Server setup, CORS config, error handling, and route definitions. Defines `FileEntry`, `ErrorResponse`, `ApiError`.
- `src/search/` — Search routes and request/response models (`SearchRequest`).
- `src/mongo/` — MongoDB integration for patient and user data (`PatientService`, `UserService`).

## Key Dependencies

- `soma_core` — All genomic search and storage functionality.
- `rocket_cors` — CORS middleware.
- `soma_core::stores::StoreService` — Cloud-agnostic file listing and access.
- `soma_core::services::search::SearchService` — Genomic query execution.
- `soma_core::sqlite::genes` — Gene symbol and coordinate lookups.

## Key Patterns

- API mirrors Tauri commands: file search, directory listing, gene lookups, patient/user CRUD.
- Uses `soma_core::utils::get_search_options()` to parse incoming search requests.
- Error responses are serialized as JSON with status codes.
- Bundled SQLite databases in `assets/` for gene and cytoband data.
