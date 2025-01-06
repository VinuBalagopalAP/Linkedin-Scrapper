// server/src/routes/api.js
const express = require("express");
const router = express.Router();
const resumeController = require("../controllers/resumeController");

/**
 * @swagger
 * /api/generate-resume:
 *   post:
 *     summary: Generate resume from LinkedIn profile
 *     description: Fetches LinkedIn profile data and generates a formatted resume
 *     tags: [Resume]
 *     requestBody:
 *       required: true
 *       content:
 *         application/json:
 *           schema:
 *             type: object
 *             required:
 *               - url
 *             properties:
 *               url:
 *                 type: string
 *                 description: LinkedIn profile URL
 *                 example: https://linkedin.com/in/username
 *     responses:
 *       200:
 *         description: Resume generated successfully
 *         content:
 *           application/json:
 *             schema:
 *               type: object
 *               properties:
 *                 firstName:
 *                   type: string
 *                 lastName:
 *                   type: string
 *                 jobTitle:
 *                   type: string
 *                 experience:
 *                   type: array
 *                   items:
 *                     type: object
 *                     properties:
 *                       title:
 *                         type: string
 *                       companyName:
 *                         type: string
 *       400:
 *         description: Invalid URL provided
 *       500:
 *         description: Server error
 */
router.post("/generate-resume", resumeController.generateResume);

module.exports = router;
