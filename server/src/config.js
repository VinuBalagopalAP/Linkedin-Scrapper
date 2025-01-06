require("dotenv").config();

module.exports = {
  PORT: process.env.PORT || 3000,
  RUST_SERVICE_URL: process.env.RUST_SERVICE_URL || "http://localhost:8080",
  JWT_SECRET: process.env.JWT_SECRET,
  RATE_LIMIT_WINDOW: 15 * 60 * 1000, // 15 minutes
  RATE_LIMIT_MAX: 100,
};
