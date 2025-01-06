const prometheus = require("prom-client");
const register = new prometheus.Registry();

const scrapeCounter = new prometheus.Counter({
  name: "linkedin_scrapes_total",
  help: "Total number of LinkedIn profile scrapes",
});

register.registerMetric(scrapeCounter);

app.get("/metrics", async (req, res) => {
  res.set("Content-Type", register.contentType);
  res.end(await register.metrics());
});
