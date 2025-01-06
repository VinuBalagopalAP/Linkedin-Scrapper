# LinkedIn Resume Generator

A microservice-based application that generates professional resumes from LinkedIn profiles.

## Documentation

### Project Structure

`server/`              → Node.js backend

├─ `src/`             → Source code

├─ `config/`          → Configuration files

├─ `controllers/`           → Route controllers

├─ `middleware/`           → Custom middleware

├─ `routes/`           → API routes

├─ `docs/`       → API documentation

├─ `tests/`           → Test files

└─ `package.json`     → Dependencies

`rust-scraper/`       → Rust scraper service  

├─ `src/`             → Source code

├─ `tests/`           → Test files

└─ `Cargo.toml`       → Dependencies

`deploy/`             → Deployment configs

├─ `docker/`          → Docker setup

### Setup & Installation

## Features

- LinkedIn profile scraping with rate limiting
- Resume generation with customizable templates
- API with authentication and request validation
- Swagger API documentation
- Docker support for easy deployment

## Getting Started

### Prerequisites

- Node.js (v14 or higher)
- Rust (latest stable)
- Docker (optional)

### Installation

#### 1. Clone the repository

```bash
git clone https://github.com/VinuBalagopalAP/Linkedin-Scrapper
```

#### 2. Install dependencies

```bash
# Node.js backend
cd server
npm install

# Rust scraper
cd ../rust-scraper
cargo build
```

#### 3. Configure environment

```bash
cp .env.example .env

# Update .env with your configurations

PORT=3000
RUST_SERVICE_URL=http://localhost:8080
JWT_SECRET=your_secret_key
```

### API Documentation

#### Endpoints

`Generate Resume`

```bash
POST /api/generate-resume
```

Request body:

```bash
{
  "url": "https://linkedin.com/in/profile"
}
```

`Health Check`

```bash
GET /health
```

Full API documentation available at `/api-docs` when server is running.

### Running Tests

```bash
# Node.js tests
cd server
npm test

# Rust tests
cd rust-scraper
cargo test
```
