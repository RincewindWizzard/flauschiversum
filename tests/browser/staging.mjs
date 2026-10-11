// Browser-Test für die Staging-Leiste.
// Voraussetzung: `flauschiversum serve --port 8098 --staging <BRANCH>` läuft im Repository,
// BRANCH wird als zweites Argument übergeben (Standard: der lange Testname unten).
// Aufruf: node tests/browser/staging.mjs [Pfad-zu-chromium] [BRANCH]
import { chromium } from 'playwright-core';
import assert from 'node:assert/strict';

const BASE = process.env.BASE ?? 'http://localhost:8098';
const executablePath = process.argv[2] ?? '/opt/pw-browsers/chromium-1194/chrome-linux/chrome';
const BRANCH = process.argv[3] ?? 'claude/ein-sehr-langer-branchname-fuer-den-handytest-0123456789abcdef-ghijklmnopqrstuvwxyz';
const browser = await chromium.launch({ executablePath });
const results = [];
const test = async (name, fn) => {
  try { await fn(); results.push(['ok', name]); }
  catch (e) { results.push(['FEHLER', name, e.message.split('\n')[0]]); }
};

for (const path of ['/', '/2020/10/kurbis/', '/kategorie/wolle/', '/suche/?q=katze', '/impressum/']) {
  await test(`Leiste ganz oben auf ${path}`, async () => {
    const page = await browser.newPage({ viewport: { width: 1280, height: 800 } });
    await page.goto(BASE + path);
    const bar = page.locator('.staging-bar');
    assert.equal(await bar.count(), 1);
    assert.equal((await bar.textContent()).trim(), `Staging: ${BRANCH}`);
    const top = await bar.evaluate((el) => el.getBoundingClientRect().top + scrollY);
    assert.equal(top, 0, 'Leiste ist das erste Element');
    assert.equal(await bar.evaluate((el) => getComputedStyle(el).backgroundColor), 'rgb(179, 38, 30)', 'Leiste bleibt rot');
    assert.match(await page.locator('meta[name=robots]').getAttribute('content'), /noindex/);
    await page.close();
  });
}

await test('Handy: langer Branchname bricht um, kein waagerechtes Scrollen', async () => {
  const page = await browser.newPage({ viewport: { width: 360, height: 740 }, hasTouch: true, isMobile: true });
  await page.goto(BASE + '/');
  const [sw, cw] = await page.evaluate(() => [document.documentElement.scrollWidth, document.documentElement.clientWidth]);
  assert.ok(sw <= cw, `scrollWidth ${sw} > clientWidth ${cw}`);
  const box = await page.locator('.staging-bar').boundingBox();
  assert.ok(box.height > 30, 'Name ist umgebrochen');
  await page.screenshot({ path: '/tmp/flausch-staging-mobile.png' });
  await page.close();
});

await test('Ohne JavaScript sichtbar', async () => {
  const ctx = await browser.newContext({ javaScriptEnabled: false });
  const page = await ctx.newPage();
  await page.goto(BASE + '/');
  assert.ok(await page.locator('.staging-bar').isVisible());
  await ctx.close();
});

await test('Absolut-URLs zeigen auf die Basis, interne Links bleiben wurzelbezogen', async () => {
  const page = await browser.newPage();
  await page.goto(BASE + '/2020/10/kurbis/');
  assert.equal(await page.locator('link[rel=canonical]').getAttribute('href'), 'https://flauschiversum.de/2020/10/kurbis/');
  assert.ok((await page.locator('nav a[href="/themen/"]').count()) >= 1);
  await page.close();
});

await browser.close();
for (const r of results) console.log(r.join(' – '));
process.exit(results.every((r) => r[0] === 'ok') ? 0 : 1);
