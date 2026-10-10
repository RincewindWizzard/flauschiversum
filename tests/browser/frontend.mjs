// Browser-Tests für Karussell, Vollbild, Videos und Live-Reload.
// Voraussetzung: `flauschiversum serve --port 8099` läuft im Repository,
// Playwright ist installiert (`npm i playwright-core`), Chromium vorhanden.
// Aufruf: node tests/browser/frontend.mjs [Pfad-zu-chromium]
import { chromium } from 'playwright-core';
import fs from 'node:fs';
import assert from 'node:assert/strict';

const BASE = process.env.BASE ?? 'http://localhost:8099';
const POST = '/2014/01/alraune/'; // Karussell mit 4 Bildern + Einzelbild
const executablePath = process.argv[2] ?? '/opt/pw-browsers/chromium-1194/chrome-linux/chrome';
const browser = await chromium.launch({ executablePath });
const results = [];
const test = async (name, fn) => {
  try { await fn(); results.push(['ok', name]); }
  catch (e) { results.push(['FEHLER', name, e.message.split('\n')[0]]); }
};

await test('Desktop: Pfeile und Punkte im Karussell', async () => {
  const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
  await page.goto(BASE + POST);
  const c = page.locator('.carousel').first();
  assert.equal(await c.locator('.carousel-dots button').count(), 4);
  assert.equal(await c.locator('.carousel-dots button[aria-current="true"]').getAttribute('aria-label'), 'Bild 1 von 4');
  assert.ok(await c.locator('.carousel-arrow.prev').isDisabled());
  await c.locator('.carousel-arrow.next').click();
  await page.waitForFunction(() => document.querySelector('.carousel-dots button[aria-current="true"]')?.getAttribute('aria-label') === 'Bild 2 von 4');
  await c.locator('.carousel-dots button').nth(3).click();
  await page.waitForFunction(() => document.querySelector('.carousel-dots button[aria-current="true"]')?.getAttribute('aria-label') === 'Bild 4 von 4');
  assert.ok(await c.locator('.carousel-arrow.next').isDisabled());
  await page.close();
});

await test('Vollbild: öffnen, blättern, Bildunterschrift, Escape', async () => {
  const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
  await page.goto(BASE + POST);
  await page.locator('.carousel-arrow.next').first().click();
  await page.waitForTimeout(600);
  await page.locator('.carousel .carousel-slide').nth(1).locator('img').click();
  await page.waitForSelector('.pswp--open');
  const src = await page.locator('.pswp__item[aria-hidden="false"] img.pswp__img:not(.pswp__img--placeholder)').first().getAttribute('src');
  assert.match(src, /1000x\/DSCF0528\.jpg$/);
  assert.equal((await page.locator('.pswp__custom-caption').textContent()).trim(), 'Alraune nah oben');
  await page.keyboard.press('ArrowRight');
  await page.waitForFunction(() => document.querySelector('.pswp__custom-caption')?.textContent === 'Alraune aus Topf gezogen');
  await page.keyboard.press('Escape');
  await page.waitForSelector('.pswp', { state: 'detached' });
  assert.equal(new URL(page.url()).pathname, POST);
  await page.screenshot({ path: '/tmp/flausch-desktop.png' });
  await page.close();
});

await test('Vollbild: Zurück-Taste schließt, Seite bleibt', async () => {
  const page = await browser.newPage({ viewport: { width: 390, height: 844 }, hasTouch: true, isMobile: true });
  await page.goto(BASE + POST);
  const img = page.locator('main > figure.media img').first();
  await img.scrollIntoViewIfNeeded();
  const box = await img.boundingBox();
  assert.ok(box.width > 100 && box.height > 100, `Bild hat Platz reserviert (${box.width}×${box.height})`);
  const y = await page.evaluate(() => scrollY);
  await img.tap();
  await page.waitForSelector('.pswp--open', { state: 'attached' });
  await page.waitForFunction(() => history.state?.pswp === true);
  // Zurück-Taste; innerhalb der Seite gibt es kein neues Laden, daher nicht darauf warten
  await page.goBack({ timeout: 3000 }).catch(() => {});
  await page.waitForSelector('.pswp', { state: 'detached' });
  assert.equal(new URL(page.url()).pathname, POST);
  assert.ok(Math.abs((await page.evaluate(() => scrollY)) - y) < 5, 'Scrollposition erhalten');
  await page.close();
});

await test('Handy: Wischen wechselt Bild, keine Pfeile', async () => {
  const page = await browser.newPage({ viewport: { width: 390, height: 844 }, hasTouch: true, isMobile: true });
  await page.goto(BASE + POST);
  const c = page.locator('.carousel').first();
  assert.ok(!(await c.locator('.carousel-arrow.next').isVisible()), 'Pfeile auf Touch ausgeblendet');
  // Wischgeste: horizontales Scrollen des Tracks
  await c.locator('.carousel-track').evaluate((t) => t.scrollTo({ left: t.clientWidth, behavior: 'instant' }));
  await page.waitForFunction(() => document.querySelector('.carousel-dots button[aria-current="true"]')?.getAttribute('aria-label') === 'Bild 2 von 4');
  await page.screenshot({ path: '/tmp/flausch-mobile.png', fullPage: false });
  await page.close();
});

await test('Ohne JavaScript: Karussell scrollbar, Bild verlinkt 1000x', async () => {
  const ctx = await browser.newContext({ javaScriptEnabled: false, viewport: { width: 390, height: 844 } });
  const page = await ctx.newPage();
  await page.goto(BASE + POST);
  const track = page.locator('.carousel-track').first();
  const [sw, cw] = await track.evaluate((t) => [t.scrollWidth, t.clientWidth]);
  assert.ok(sw >= cw * 3.9, `scrollbar (${sw} / ${cw})`);
  await track.evaluate((t) => { t.scrollLeft = t.clientWidth * 3; });
  await page.waitForTimeout(800); // sanftes Scrollen abwarten
  assert.ok((await track.evaluate((t) => t.scrollLeft)) > 0);
  assert.match(await page.locator('.carousel a').first().getAttribute('href'), /^1000x\//);
  assert.ok((await page.locator('main p').count()) > 0, 'Text sichtbar');
  await ctx.close();
});

await test('Bewegung reduzieren: Video startet nicht selbst', async () => {
  const ctx = await browser.newContext({ reducedMotion: 'reduce' });
  const page = await ctx.newPage();
  // Ein Video in die Seite einfügen (kein bestehender Post bindet eins ein)
  await page.route(BASE + POST, async (route) => {
    const res = await route.fetch();
    const body = (await res.text()).replace('<main class="page">',
      '<main class="page"><figure class="media media-video"><video id="v" src="/2014/05/karteikartenfresser/VID_20130115_184912.mp4" autoplay muted loop playsinline></video></figure>');
    await route.fulfill({ response: res, body });
  });
  await page.goto(BASE + POST);
  await page.waitForTimeout(800);
  const [paused, controls, auto] = await page.locator('#v').evaluate((v) => [v.paused, v.controls, v.autoplay]);
  assert.ok(paused && controls && !auto, `paused=${paused} controls=${controls} autoplay=${auto}`);
  await ctx.close();
});

await test('Cache-Busting: CSS und JS mit Inhalts-Hash', async () => {
  const page = await browser.newPage();
  await page.goto(BASE + POST);
  assert.match(await page.locator('link[rel=stylesheet][href^="/style.css"]').getAttribute('href'), /^\/style\.css\?v=[0-9a-f]{10}$/);
  assert.match(await page.locator('script[type=module]').getAttribute('src'), /^\/assets\/js\/media\.js\?v=[0-9a-f]{10}$/);
  await page.close();
});

await test('Live-Reload: Änderung erscheint ohne Neuladen', async () => {
  const file = new URL('../../content/pages/impressum.md', import.meta.url);
  const original = fs.readFileSync(file, 'utf8');
  const page = await browser.newPage();
  await page.goto(BASE + '/impressum/');
  try {
    fs.writeFileSync(file, original + '\n\nLive-Reload-Testzeile\n');
    await page.waitForSelector('text=Live-Reload-Testzeile', { timeout: 10000 });
  } finally {
    fs.writeFileSync(file, original);
  }
  await page.close();
});

await browser.close();
for (const r of results) console.log(r.join(' – '));
process.exit(results.every((r) => r[0] === 'ok') ? 0 : 1);
