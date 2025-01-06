const express = require("express");
const config = require("./config/config");
const auth = require("./middleware/auth");
const rateLimiter = require("./middleware/rateLimiter");
const errorHandler = require("./middleware/errorHandler");
const { validateLinkedInUrl } = require("./middleware/validator");
const apiRoutes = require("./routes/api");

const app = express();

app.use(express.json());
app.use(rateLimiter);
app.use("/api", auth, validateLinkedInUrl, apiRoutes);
app.use(errorHandler);

app.listen(config.PORT, () => {
  console.log(`Server running on port ${config.PORT}`);
});
