const rateLimit = require("express-rate-limit");
const { RATE_LIMIT_WINDOW, RATE_LIMIT_MAX } = require("../config");

module.exports = rateLimit({
  windowMs: RATE_LIMIT_WINDOW,
  max: RATE_LIMIT_MAX,
});
