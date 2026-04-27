const fs = require("fs");
const path = require("path");
const https = require("https");

const platform = process.platform;
let version;

try {
  version = require("./package.json").version;
} catch (err) {
  console.error("Failed to read package.json version:", err.message);
  process.exit(1);
}

const platformMap = {
  win32: { target: "x86_64-pc-windows-msvc", ext: ".exe" },
  darwin: { target: "aarch64-apple-darwin", ext: "" },
  linux: { target: "x86_64-unknown-linux-gnu", ext: "" },
};

if (!platformMap[platform]) {
  console.error(`Unsupported platform: ${platform}`);
  process.exit(1);
}

const { target, ext } = platformMap[platform];
const filename = `nrmrs-v${version}-${target}${ext}`;
const url = `https://github.com/MissGwen/nrmrs/releases/download/v${version}/${filename}`;
const binDir = path.join(__dirname, "bin");
const binPath = path.join(binDir, platform === "win32" ? "nrmrs.exe" : "nrmrs");

// Check if already installed
if (fs.existsSync(binPath)) {
  console.log(`Binary already exists at ${binPath}`);
  process.exit(0);
}

if (!fs.existsSync(binDir)) {
  fs.mkdirSync(binDir, { recursive: true });
}

console.log(`Downloading ${filename}...`);

function downloadFile(downloadUrl, destPath) {
  https
    .get(downloadUrl, (response) => {
      // GitHub Releases redirect to S3 via 301/302
      if (response.statusCode === 301 || response.statusCode === 302) {
        return downloadFile(response.headers.location, destPath);
      }

      if (response.statusCode !== 200) {
        console.error(`Failed to download: HTTP ${response.statusCode}`);
        if (fs.existsSync(destPath)) fs.unlinkSync(destPath);
        process.exit(1);
      }

      const file = fs.createWriteStream(destPath);
      response.pipe(file);
      
      file.on("finish", () => {
        file.close();

        // Unix上设置执行权限
        if (platform !== "win32") {
          fs.chmodSync(destPath, "0755");
        }

        console.log(`Successfully installed ${path.basename(destPath)}`);
      });

      file.on("error", (err) => {
        if (fs.existsSync(destPath)) fs.unlinkSync(destPath);
        console.error("File write error:", err.message);
        process.exit(1);
      });
    })
    .on("error", (err) => {
      console.error("Download error:", err.message);
      process.exit(1);
    });
}

downloadFile(url, binPath);
