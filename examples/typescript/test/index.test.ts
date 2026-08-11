import { describe, expect, it } from "vitest";
import { greet } from "../src/index.js";

describe("greet", () => {
  it("returns a friendly greeting", () => {
    expect(greet("Sheridan")).toBe("Hello, Sheridan!");
  });

  it("rejects an empty name", () => {
    expect(() => greet("")).toThrow("name must not be empty");
  });
});
