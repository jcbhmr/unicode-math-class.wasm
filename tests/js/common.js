import { readFile } from "node:fs/promises";
import { instantiate } from "./out/unicode-math-class.js";

export const test = await import("node:test")
  .then((m) => m.default)
  .catch(() => import("bun:test").then((m) => m.test));

async function getCoreModule(filename) {
  const fileURL = await import.meta.resolve(`./out/${filename}`);
  const buffer = await readFile(new URL(fileURL));
  const bytes = new Uint8Array(
    buffer.buffer,
    buffer.byteOffset,
    buffer.byteLength
  );
  return await WebAssembly.compile(bytes);
}

/**
 * @param {import('./out/unicode-math-class.js').Imports} importObject
 * @returns {Promise<import('./out/unicode-math-class.js').Root>}
 */
export const instantiate2 = async (importObject) =>
  await instantiate(getCoreModule, importObject);
