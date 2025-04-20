import crypto, { pbkdf2Sync } from "crypto";

export function usernameToColor(username: string): string {
  let hash = 0;

  for (let i = 0; i < username.length; i++) {
    hash = username.charCodeAt(i) + ((hash << 5) - hash);
  }

  const hue = Math.abs(hash) % 360;

  return `hsl(${hue}, 60%, 60%)`;
}

export function generateProfilePictureSVG(username: string) {
  const gridSize = 10;
  const cellSize = 50;

  const width = gridSize * cellSize;
  const height = gridSize * cellSize;

  const bgColor = "#F0F0F0";
  const patternColor = `${usernameToColor(username)}`;

  const halfCols = Math.ceil(gridSize / 2);
  const pattern: boolean[][] = [];

  for (let row = 0; row < gridSize; row++) {
    pattern[row] = [];
    for (let col = 0; col < halfCols; col++) {
      pattern[row][col] = Math.random() < 0.5;
    }
    for (let col = halfCols; col < gridSize; col++) {
      pattern[row][col] = pattern[row][gridSize - col - 1];
    }
  }

  let svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}" viewBox="0 0 ${width} ${height}">`;
  svg += `<rect width="${width}" height="${height}" fill="${bgColor}" />`;

  for (let row = 0; row < gridSize; row++) {
    for (let col = 0; col < gridSize; col++) {
      if (pattern[row][col]) {
        const x = col * cellSize;
        const y = row * cellSize;
        svg += `<rect x="${x}" y="${y}" width="${cellSize}" height="${cellSize}" fill="${patternColor}" />`;
      }
    }
  }

  svg += `</svg>`;
  const base64Svg = btoa(svg);
  return `data:image/svg+xml;base64,${base64Svg}`;
}

export function getIdFromJWT() {
  const token = localStorage.getItem("token") ?? "";
  const base64Url = token.split(".")[1];
  const base64 = base64Url.replace(/-/g, "+").replace(/_/g, "/");
  const jsonPayload = decodeURIComponent(
    atob(base64)
      .split("")
      .map((c) => {
        return "%" + ("00" + c.charCodeAt(0).toString(16)).slice(-2);
      })
      .join("")
  );

  return parseInt(JSON.parse(jsonPayload).sub);
}

export function convertToEndDate(minutes: number): string {
  const now = new Date();
  const futureDate = new Date(now.getTime() + minutes * 60 * 1000);
  return futureDate.toISOString();
}

export function deriveEncryptionKey(password: string, salt: string) {
  return pbkdf2Sync(password, salt, 100000, 32, "sha256");
}

export function encryptMessage(message: string, key: Buffer) {
  const cipher = crypto.createCipheriv("aes-256-ecb", key, Buffer.alloc(0));
  let encryptedMessage = cipher.update(message, "utf8", "hex");
  encryptedMessage += cipher.final("hex");
  return encryptedMessage;
}

export function decryptMessage(encryptedMessage: string, key: Buffer) {
  const decipher = crypto.createDecipheriv("aes-256-ecb", key, Buffer.alloc(0));
  let decryptedMessage = decipher.update(encryptedMessage, "hex", "utf8");
  decryptedMessage += decipher.final("utf8");
  return decryptedMessage;
}
