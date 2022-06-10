import * as exec from "@actions/exec";
import path from "path";

async function run() {
  const script = path.normalize(
    path.join(__dirname, "..", "src", "reset-directory-mtime")
  );
  try {
    await exec.exec("python", [script]);
  } catch (e) {
    console.error("reset-directory-mtime failed:\n", e);
    process.exit(1);
  }
}

run();
