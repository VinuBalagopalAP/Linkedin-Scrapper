const swaggerJsdoc = require("swagger-jsdoc");

const options = {
  definition: {
    openapi: "3.0.0",
    info: {
      title: "LinkedIn Resume Generator API",
      version: "1.0.0",
      description: "API for generating resumes from LinkedIn profiles",
    },
    servers: [
      {
        url: "http://localhost:3000",
        description: "Development server",
      },
    ],
  },
  apis: [
    "./src/routes/*.js", // API routes
    "./src/controllers/*.js", // Controllers
    "./src/models/*.js", // Models/schemas if any
  ],
};

module.exports = swaggerJsdoc(options);
