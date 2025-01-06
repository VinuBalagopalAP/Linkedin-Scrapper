const request = require("supertest");
const app = require("../src/app");

describe("Resume API", () => {
  test("should generate resume", async () => {
    const res = await request(app)
      .post("/api/generate-resume")
      .send({ url: "test-url" });
    expect(res.status).toBe(200);
  });
});
