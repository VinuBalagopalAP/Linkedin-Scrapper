const request = require("supertest");
const app = require("../src/app");

describe("Resume API", () => {
  // Test authentication
  test("should require authentication", async () => {
    const response = await request(app).post("/api/generate-resume");
    expect(response.status).toBe(401);
  });

  // Test resume generation
  test("should generate resume with valid LinkedIn URL", async () => {
    const response = await request(app)
      .post("/api/generate-resume")
      .set("Authorization", `Bearer ${validToken}`)
      .send({
        url: "https://linkedin.com/in/test-profile",
      });
    expect(response.status).toBe(200);
    expect(response.body).toHaveProperty("firstName");
  });
});
