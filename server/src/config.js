// server/src/config.js
require("dotenv").config();

module.exports = {
  PORT: process.env.PORT || 3000,
  RUST_SERVICE_URL: process.env.RUST_SERVICE_URL || "http://rust-scraper:8080", // Update for Docker network
  JWT_SECRET: process.env.JWT_SECRET || "default-jwt-secret-key",
  RATE_LIMIT_WINDOW: parseInt(process.env.RATE_LIMIT_WINDOW) || 15 * 60 * 1000, // 15 minutes
  RATE_LIMIT_MAX: parseInt(process.env.RATE_LIMIT_MAX) || 100,
  NODE_ENV: process.env.NODE_ENV || "development",
  LOG_LEVEL: process.env.LOG_LEVEL || "info",
};
