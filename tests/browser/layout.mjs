// Browser-Tests für Spalten (Navigation, Sidebar), Footer und Autor.
// Voraussetzung: `flauschiversum serve --port 8099` läuft im Repository,
// Playwright ist installiert (`npm i playwright-core`), Chromium vorhanden.
// Aufruf: node tests/browser/layout.mjs [Pfad-zu-chromium]
import { chromium } from 'playwright-core';
import assert from 'node:assert/strict';

const BASE = process.env.BASE ?? 'http://localhost:8099';
const executablePath = process.argv[2] ?? '/opt/pw-browsers/chromium-1194/chrome-linux/chrome';
const POST = '/2020/10/kurbis/';
const browser = await chromium.launch({ executablePath });
const results = [];
const test = async (name, fn) => {
  try { await fn(); results.push(['ok', name]); }
  catch (e) { results.push(['FEHLER', name, e.message.split('\n')[0]]); }
};
const open = async (path, width, height = 900, extra = {}) => {
  const page = await browser.newPage({ viewport: { width, height }, ...extra });
  await page.goto(BASE + path);
  return page;
};
const box = async (page, sel) => page.locator(sel).first().boundingBox();
const visibleCount = (page, sel) => page.locator(`${sel}:visible`).count();

await test('1440 px: drei Spalten, Kopf nur mit Logo', async () => {
  const page = await open(POST, 1440);
  const [nav, content, side] = [await box(page, '.sitenav'), await box(page, 'main.page'), await box(page, '.sidebar')];
  assert.ok(nav && content && side, 'alle drei Spalten sichtbar');
  assert.ok(nav.x + nav.width <= content.x && content.x + content.width <= side.x, 'nebeneinander: Navigation, Inhalt, Sidebar');
  assert.ok(content.width <= 40 * 16 + 40, `Lesetext höchstens etwa 40 em (${content.width} px)`);
  assert.equal(await visibleCount(page, '#blogheader .topnav'), 0, 'Navigationsleiste im Kopf ausgeblendet');
  assert.equal(await visibleCount(page, '#blogheader .topsearch'), 0, 'Suchfeld im Kopf ausgeblendet');
  assert.ok(await page.locator('#blogheader .logo').isVisible());
  // Reihenfolge der linken Spalte: Suchfeld, Start, Themen, Beiträge, Kategorien
  const ys = [];
  for (const sel of ['.sitenav .sidesearch', '.sitenav .sitenav-links li:nth-child(1)', '.sitenav .sitenav-links li:nth-child(3)', '.sitenav .cats']) ys.push((await box(page, sel)).y);
  assert.deepEqual(ys, [...ys].sort((a, b) => a - b));
  await page.screenshot({ path: '/tmp/flausch-layout-1440.png' });
  await page.close();
});

await test('1440 px: genau ein sichtbares Suchfeld und eine sichtbare Kategorienliste', async () => {
  for (const path of [POST, '/kategorie/filzen/', '/impressum/']) {
    const page = await open(path, 1440);
    assert.equal(await visibleCount(page, 'input[type=search]'), 1, path);
    assert.equal(await visibleCount(page, 'ul.cats'), 1, path);
    await page.close();
  }
});

await test('1440 px: Navigationsspalte bleibt beim Scrollen sichtbar', async () => {
  const page = await open(POST, 1440, 900);
  await page.evaluate(() => scrollTo(0, 1500));
  await page.waitForTimeout(200);
  const nav = await box(page, '.sitenav');
  assert.ok(nav.y >= 0 && nav.y < 200, `Spalte klebt oben (y=${nav.y})`);
  await page.close();
});

await test('1100 px: zwei Spalten, Navigation im Kopf, Kategorien in der Sidebar', async () => {
  const page = await open(POST, 1100);
  const [content, side] = [await box(page, 'main.page'), await box(page, '.sidebar')];
  assert.ok(content.x + content.width <= side.x, 'Sidebar rechts neben dem Inhalt');
  assert.equal(await visibleCount(page, '.sitenav'), 0);
  assert.equal(await visibleCount(page, '#blogheader .topnav'), 1);
  assert.equal(await visibleCount(page, 'input[type=search]'), 1);
  assert.equal(await visibleCount(page, 'ul.cats'), 1);
  assert.equal(await visibleCount(page, '.sidebar .cats'), 1);
  await page.close();
});

await test('360 px: Sidebar nach dem Inhalt und vor dem Footer, kein waagerechtes Scrollen', async () => {
  const page = await open(POST, 360, 740, { hasTouch: true, isMobile: true });
  const [content, side, footer] = [await box(page, 'main.page'), await box(page, '.sidebar'), await box(page, '#blogfooter')];
  assert.ok(content.y + content.height <= side.y + 1 && side.y + side.height <= footer.y + 1, 'Reihenfolge Inhalt, Sidebar, Footer');
  const [sw, cw] = await page.evaluate(() => [document.documentElement.scrollWidth, document.documentElement.clientWidth]);
  assert.ok(sw <= cw, `scrollWidth ${sw} > clientWidth ${cw}`);
  assert.equal(await visibleCount(page, 'input[type=search]'), 1);
  await page.close();
});

await test('Suche und Themenübersicht: einspaltig ohne Seitenspalten', async () => {
  for (const path of ['/suche/?q=katze', '/themen/']) {
    const page = await open(path, 1440);
    assert.equal(await visibleCount(page, '.sidebar, .sitenav'), 0, path);
    assert.equal(await visibleCount(page, '#blogheader .topnav'), 1, path);
    await page.close();
  }
});

await test('Tag-Cloud: Größe nach Häufigkeit, Link führt auf die Schlagwortseite', async () => {
  const page = await open('/', 1440);
  const size = async (name) => page.locator('.cloud a', { hasText: new RegExp(`^${name}$`) }).evaluate((el) => parseFloat(getComputedStyle(el).fontSize));
  const base = await page.locator('.cloud').evaluate((el) => parseFloat(getComputedStyle(el).fontSize));
  assert.ok((await size('Hüte')) > (await size('Weihnachten')), 'häufiger = größer');
  const all = await page.locator('.cloud a').evaluateAll((els) => els.map((e) => parseFloat(getComputedStyle(e).fontSize)));
  assert.ok(Math.min(...all) / base >= 0.84 && Math.max(...all) / base <= 1.61, `Größen zwischen etwa 0,85 und 1,6 em (${Math.min(...all) / base}–${Math.max(...all) / base})`);
  const names = await page.locator('.cloud a').allTextContents();
  assert.deepEqual(names, [...names].sort((a, b) => a.localeCompare(b, 'de')), 'alphabetisch');
  await page.locator('.cloud a', { hasText: /^Hüte$/ }).click();
  await page.waitForURL('**/tag/huete/');
  await page.close();
});

await test('Autor am Ende: nach dem Text, vor älterem Beitrag, nicht im Kopf', async () => {
  const page = await open(POST, 1280);
  assert.equal(await page.locator('.post-header .author').count(), 0);
  const author = await box(page, 'main.page p.author');
  const last = await box(page, 'main.page > :nth-last-child(2)');
  const postNav = await box(page, 'nav.post-nav');
  assert.ok(author.y >= last.y, 'nach dem letzten Absatz');
  assert.ok(author.y < postNav.y, 'vor den Links zum älteren Beitrag');
  assert.equal(await page.locator('main.page p.author a').getAttribute('href'), '/about/');
  await page.close();
});

for (const [width, label] of [[1280, 'Desktop'], [360, 'Handy']]) {
  await test(`Footer ${label}: letztes Element, alle Links sichtbar, Klickflächen`, async () => {
    const page = await open(POST, width, 800, width < 500 ? { hasTouch: true, isMobile: true } : {});
    const footer = page.locator('#blogfooter');
    const fb = await footer.boundingBox();
    const docH = await page.evaluate(() => document.documentElement.scrollHeight);
    assert.ok(Math.abs(fb.y + fb.height - docH) < 60, 'Footer steht am Seitenende');
    for (const name of ['Tipps und Tricks', 'About', 'Impressum', 'Abonnieren']) {
      assert.ok(await footer.getByRole('link', { name }).isVisible(), name);
    }
    for (const [name, href] of [['Instagram', 'https://www.instagram.com/ermelinewollknoll/'], ['Mastodon', 'https://chaos.social/@ErmelineWollknoll']]) {
      const link = footer.getByRole('link', { name });
      assert.ok(await link.isVisible(), name);
      assert.equal(await link.getAttribute('href'), href);
      const b = await link.boundingBox();
      assert.ok(b.width >= 44 && b.height >= 44, `Klickfläche ${name}: ${b.width}×${b.height}`);
    }
    const mail = await footer.locator('a.mail').getAttribute('href');
    assert.equal(mail, 'mailto:ermeline@flauschiversum.de?subject=Flauschiversum');
    const [sw, cw] = await page.evaluate(() => [document.documentElement.scrollWidth, document.documentElement.clientWidth]);
    assert.ok(sw <= cw, 'kein waagerechtes Scrollen');
    if (width > 500) await footer.screenshot({ path: '/tmp/flausch-footer.png' });
    await page.close();
  });
}

await test('Footer: Tastatur erreicht die Icons, Fokus ist sichtbar', async () => {
  const page = await open(POST, 1280);
  await page.getByRole('link', { name: 'Instagram' }).focus();
  await page.keyboard.press('Tab');
  const active = await page.evaluate(() => document.activeElement?.getAttribute('aria-label'));
  assert.equal(active, 'Mastodon');
  const outline = await page.evaluate(() => { const s = getComputedStyle(document.activeElement); return [s.outlineStyle, parseFloat(s.outlineWidth)]; });
  assert.ok(outline[0] !== 'none' && outline[1] > 0, `Fokusrahmen ${outline}`);
  await page.close();
});

await test('Footer: Mastodon-Link trägt rel=me, externe Links rel=noopener', async () => {
  const page = await open('/', 1280);
  assert.match(await page.getByRole('link', { name: 'Mastodon' }).getAttribute('rel'), /\bme\b.*\bnoopener\b/);
  assert.match(await page.getByRole('link', { name: 'Instagram' }).getAttribute('rel'), /noopener/);
  await page.close();
});

for (const path of ['/', POST, '/kategorie/filzen/', '/impressum/', '/suche/?q=katze']) {
  await test(`Keine Anfragen an fremde Server: ${path}`, async () => {
    const page = await browser.newPage();
    const hosts = new Set();
    page.on('request', (r) => hosts.add(new URL(r.url()).host));
    await page.goto(BASE + path);
    await page.waitForTimeout(800);
    assert.deepEqual([...hosts], [new URL(BASE).host]);
    await page.close();
  });
}

await test('Alle mailto-Links haben „Flauschiversum“ im Betreff', async () => {
  for (const path of [POST, '/impressum/', '/']) {
    const page = await open(path, 1280);
    const hrefs = await page.locator('a[href^="mailto:"]').evaluateAll((els) => els.map((e) => e.getAttribute('href')));
    assert.ok(hrefs.length >= 1, path);
    for (const h of hrefs) assert.match(decodeURIComponent(h.split('subject=')[1] ?? ''), /Flauschiversum/, `${path}: ${h}`);
    await page.close();
  }
});

await browser.close();
for (const r of results) console.log(r.join(' – '));
process.exit(results.every((r) => r[0] === 'ok') ? 0 : 1);
