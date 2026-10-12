// Erzeugt favicon.ico (16, 32, 48 px) und apple-touch-icon.png (180 px) aus content/static/favicon.svg.
// Aufruf aus dem Projektverzeichnis: node <Pfad>/make_favicons.mjs   (Playwright mit Chromium und ImageMagick `convert` nötig;
// mit `--preview` entstehen zusätzlich Vorschaubilder unter /tmp/favicon-preview-*.png)
import { chromium } from 'playwright-core';
import { readFileSync, writeFileSync, mkdtempSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const svg = readFileSync('content/static/favicon.svg', 'utf8');
const exe = process.env.CHROMIUM ?? '/opt/pw-browsers/chromium-1194/chrome-linux/chrome';
const browser = await chromium.launch({ executablePath: exe });
const dir = mkdtempSync(join(tmpdir(), 'favicon-'));

const render = async (size, background = 'transparent', file) => {
  const page = await browser.newPage({ viewport: { width: size, height: size }, deviceScaleFactor: 1 });
  await page.setContent(`<style>html,body{margin:0;background:${background}}svg{display:block;width:${size}px;height:${size}px}</style>${svg}`);
  await page.screenshot({ path: file, omitBackground: background === 'transparent' });
  await page.close();
  return file;
};

// Zielgröße ohne Rand; iOS rundet die Ecken selbst, die Fläche ist deshalb quadratisch und deckend (Lavendel, `--bgcolor`)
const png = {};
for (const s of [16, 32, 48]) png[s] = await render(s, 'transparent', join(dir, `${s}.png`));
const apple = await (async () => {
  const page = await browser.newPage({ viewport: { width: 180, height: 180 } });
  await page.setContent(`<style>html,body{margin:0;background:#f6f3fc}svg{display:block;width:180px;height:180px;padding:0}</style><div style="width:180px;height:180px;background:#f6f3fc;display:grid;place-items:center"><div style="width:148px;height:148px">${svg.replace('<svg ', '<svg width="148" height="148" ')}</div></div>`);
  await page.screenshot({ path: 'content/static/apple-touch-icon.png' });
  await page.close();
})();
execFileSync('convert', [png[16], png[32], png[48], 'content/static/favicon.ico']);

if (process.argv.includes('--preview')) {
  for (const [name, bg] of [['hell', '#ffffff'], ['dunkel', '#1a1a1a']]) {
    const page = await browser.newPage({ viewport: { width: 520, height: 200 } });
    const img = (s, scale) => `<svg style="width:${s * scale}px;height:${s * scale}px;image-rendering:pixelated"></svg>`;
    const data = (s) => `data:image/png;base64,${readFileSync(png[s]).toString('base64')}`;
    await page.setContent(`<body style="margin:0;background:${bg};display:flex;gap:24px;align-items:flex-end;padding:20px;font:12px sans-serif;color:#888"><div>16 px ×8<br><img src="${data(16)}" style="width:128px;image-rendering:pixelated"></div><div>16 px echt<br><img src="${data(16)}"></div><div>32 px<br><img src="${data(32)}"></div><div>48 px<br><img src="${data(48)}"></div><div>SVG 96<br><div style="width:96px;height:96px">${svg}</div></div></body>`);
    await page.screenshot({ path: `/tmp/favicon-preview-${name}.png` });
    await page.close();
  }
}
await browser.close();
writeFileSync(join(dir, 'ok'), '');
