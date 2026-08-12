#!/usr/bin/env node
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { spawnSync } from "node:child_process";

const BIN_NAME = "ts-pack";

function binaryName() {
  return os.type() === "Windows_NT" ? `${BIN_NAME}.exe` : BIN_NAME;
}

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const binPath = path.join(__dirname, binaryName());

function isHealthy(file) {
  try {
    const stat = fs.statSync(file);
    if (stat.size <= 0) return false;
    if (os.type() !== "Windows_NT" && (stat.mode & 0o111) === 0) return false;
    return true;
  } catch {
    return false;
  }
}

async function ensureBinary() {
  if (fs.existsSync(binPath) && isHealthy(binPath)) return;
  process.stderr.write(`${BIN_NAME}: binary missing or corrupt, attempting download...\n`);
  const { main: runInstaller } = await import("../install.js");
  await runInstaller();
}

function printUnavailable() {
  process.stderr.write(
    `${BIN_NAME} is not available for your platform yet. Install it with:\n` +
      `  brew install xberg-io/tap/ts-pack\n` +
      `  or use the Xberg plugin:  /plugin marketplace add xberg-io/plugins\n`,
  );
}

async function main() {
  await ensureBinary();
  if (!fs.existsSync(binPath) || !isHealthy(binPath)) {
    printUnavailable();
    process.exit(1);
  }
  const result = spawnSync(binPath, process.argv.slice(2), { stdio: "inherit" });
  if (result.error) {
    process.stderr.write(`${BIN_NAME}: failed to spawn binary: ${result.error.message}\n`);
    process.exit(1);
  }
  process.exit(result.status ?? 0);
}

main().catch((err) => {
  if (err && err.name === "CliUnavailableError") {
    printUnavailable();
    process.exit(1);
  }
  process.stderr.write(`${BIN_NAME}: ${err.message}\n`);
  process.exit(1);
});
