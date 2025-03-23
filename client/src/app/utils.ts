export function usernameToColor(username: string): string {
  let hash = 0;

  for (let i = 0; i < username.length; i++) {
    hash = username.charCodeAt(i) + ((hash << 5) - hash);
  }

  const hue = Math.abs(hash) % 360;

  return `hsl(${hue}, 70%, 70%)`;
}

export function generateProfilePictureSVG() {
  const gridSize = 10;
  const cellSize = 50;

  const width = gridSize * cellSize;
  const height = gridSize * cellSize;

  const bgColor = "#F0F0F0";
  const patternColor = `${usernameToColor("username")}`;

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
