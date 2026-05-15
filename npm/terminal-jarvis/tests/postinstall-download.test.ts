import { PassThrough } from "stream";
import { mkdtemp, readFile, rm } from "fs/promises";
import { tmpdir } from "os";
import path from "path";
import { afterEach, describe, expect, test, vi } from "vitest";

const https = require("https");
const { download } = require("../scripts/postinstall.js");

describe("postinstall download", () => {
  const originalGet = https.get;
  let tempDir: string | undefined;

  afterEach(async () => {
    https.get = originalGet;
    vi.restoreAllMocks();

    if (tempDir) {
      await rm(tempDir, { recursive: true, force: true });
      tempDir = undefined;
    }
  });

  test("drains GitHub redirect responses before following asset URL", async () => {
    tempDir = await mkdtemp(path.join(tmpdir(), "terminal-jarvis-download-"));
    const destination = path.join(tempDir, "asset.tar.gz");
    const redirectResume = vi.fn();
    const seenUrls: string[] = [];

    https.get = vi.fn((url: string, callback: (response: PassThrough) => void) => {
      seenUrls.push(url);

      const response = new PassThrough() as PassThrough & {
        statusCode: number;
        statusMessage: string;
        headers: Record<string, string>;
        resume: () => PassThrough;
      };

      if (seenUrls.length === 1) {
        response.statusCode = 302;
        response.statusMessage = "Found";
        response.headers = { location: "https://downloads.example/asset.tar.gz" };
        response.resume = vi.fn(() => {
          redirectResume();
          return response;
        });
        queueMicrotask(() => callback(response));
      } else {
        response.statusCode = 200;
        response.statusMessage = "OK";
        response.headers = { "content-length": "7" };
        queueMicrotask(() => {
          callback(response);
          response.end("archive");
        });
      }

      return { on: vi.fn() };
    });

    await download("https://github.example/release", destination, 1);

    expect(seenUrls).toEqual([
      "https://github.example/release",
      "https://downloads.example/asset.tar.gz",
    ]);
    expect(redirectResume).toHaveBeenCalledOnce();
    await expect(readFile(destination, "utf8")).resolves.toBe("archive");
  });
});
