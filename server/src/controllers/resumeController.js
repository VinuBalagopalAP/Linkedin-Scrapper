const { getLinkedInProfile } = require("../services/linkedinService");
const { transformToResumeFormat } = require("../utils/transformer");

exports.generateResume = async (req, res) => {
  try {
    const { url } = req.body;
    const profile = await getLinkedInProfile(url);
    const resumeData = transformToResumeFormat(profile);
    res.json(resumeData);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
};
