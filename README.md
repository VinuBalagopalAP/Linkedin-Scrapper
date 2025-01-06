# LinkedIn Resume Generator

## Documentation

### Project Structure

`server/`              → Node.js backend

├─ `src/`             → Source code

├─ `tests/`           → Test files

├─ `config/`          → Configuration files

└─ `package.json`     → Dependencies

`rust-scraper/`       → Rust scraper service  

├─ `src/`             → Source code

├─ `tests/`           → Test files

└─ `Cargo.toml`       → Dependencies

`deploy/`             → Deployment configs

├─ `docker/`          → Docker setup

### Setup & Installation

1. `npm install`
2. `cargo build`
3. Configure `.env` files

### API Documentation

- POST `/api/generate-resume` - Generate resume from LinkedIn URL
- Health `/health` - Health check endpoint
