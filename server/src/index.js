// server/src/index.js
const express = require("express");
const swaggerUi = require("swagger-ui-express");
const swaggerSpec = require("./docs/swagger");
const config = require("./config/config");
const auth = require("./middleware/auth");
const rateLimiter = require("./middleware/rateLimiter");
const errorHandler = require("./middleware/errorHandler");
const { validateLinkedInUrl } = require("./middleware/validator");
const apiRoutes = require("./routes/api");

const app = express();

// Middleware
app.use(express.json());
app.use(rateLimiter);

// Swagger documentation
app.use("/api-docs", swaggerUi.serve);
app.get("/api-docs", swaggerUi.setup(swaggerSpec));

// API routes
app.use("/api", auth, validateLinkedInUrl, apiRoutes);

// Error handling
app.use(errorHandler);

// Health check endpoint
app.get("/health", (req, res) => {
  res.status(200).json({ status: "ok" });
});

app.listen(config.PORT, () => {
  console.log(`Server running on port ${config.PORT}`);
  console.log(
    `API Documentation available at http://localhost:${config.PORT}/api-docs`
  );
});
