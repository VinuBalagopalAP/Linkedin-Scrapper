const axios = require("axios");
const config = require("../config");

exports.getLinkedInProfile = async (url) => {
  try {
    const response = await axios.post(`${config.RUST_SERVICE_URL}/scrape`, {
      url,
    });
    return response.data;
  } catch (error) {
    throw new Error("Failed to fetch LinkedIn profile");
  }
};
