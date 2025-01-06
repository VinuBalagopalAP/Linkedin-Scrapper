const { validationResult } = require("express-validator");

exports.validateLinkedInUrl = [
  body("url").isURL().contains("linkedin.com"),
  (req, res, next) => {
    const errors = validationResult(req);
    if (!errors.isEmpty()) {
      return res.status(400).json({ errors: errors.array() });
    }
    next();
  },
];
