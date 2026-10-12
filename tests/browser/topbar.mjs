// Browser-Tests für Topbar (Maskottchen, Menüs, Handy), Archiv und Seiten mit Archiv.
// Voraussetzung: `flauschiversum serve --port 8099` läuft im Repository,
// Playwright ist installiert (`npm i playwright-core`), Chromium vorhanden.
// Aufruf: node tests/browser/topbar.mjs [Pfad-zu-chromium]
import { chromium } from 'playwright-core';
import assert from 'node:assert/strict';

const BASE = process.env.BASE ?? 'http://localhost:8099';
const executablePath = process.argv[2] ?? '/opt/pw-browsers/chromium-1194/chrome-linux/chrome';
const browser = await chromium.launch({ executablePath });
const results = [];
const test = async (name, fn) => {
  try { await fn(); results.push(['ok', name]); }
  catch (e) { results.push(['FEHLER', name, e.message.split('\n')[0]]); }
};
const open = async (path, width = 1280, height = 900, extra = {}) => {
  const page = await browser.newPage({ viewport: { width, height }, ...extra });
  await page.goto(BASE + path);
  return page;
};
const box = (page, sel) => page.locator(sel).first().boundingBox();
const noHScroll = async (page, label) => {
  const [sw, cw] = await page.evaluate(() => [document.documentElement.scrollWidth, document.documentElement.clientWidth]);
  assert.ok(sw <= cw, `${label}: scrollWidth ${sw} > clientWidth ${cw}`);
};

await test('Topbar am Desktop: Maskottchen und Titel links, rechts Beiträge, Themen, Tipps und Tricks, Suche – alles auf einer Linie', async () => {
  const page = await open('/2020/10/kurbis/', 1280);
  const [mascot, brand, posts, cats, tips, search, bar] = [await box(page, '.mascot'), await box(page, '.brand-name'), await box(page, '.topnav a[href="/beitraege/"]'), await box(page, '.topnav a[href="/themen/"]'), await box(page, '.topnav a[href="/tipps-und-tricks/"]'), await box(page, '.topsearch'), await box(page, '.topbar-inner')];
  assert.ok(mascot.x < brand.x && brand.x + brand.width < posts.x, 'Maskottchen, Titel, dann Menüs');
  assert.ok(posts.x < cats.x && cats.x < tips.x && tips.x + tips.width <= search.x + 1, 'Reihenfolge der Menüs, Suche zuletzt');
  assert.ok(mascot.x < 200, 'ganz links');
  const toggle = await box(page, '.theme-toggle');
  assert.ok(toggle.x + toggle.width > 1280 - 100 && posts.x > 300, 'Menüs rechtsbündig');
  assert.ok(search.x + search.width <= toggle.x + 1, 'Umschalter nach der Suche');
  assert.ok(Math.abs((toggle.y + toggle.height / 2) - (bar.y + bar.height / 2)) <= 2, 'Umschalter auf der Mittellinie');
  assert.equal(await page.locator('.topnav a[href="/about/"]').count(), 0, 'About me steht nicht in der Topbar');
  // vertikal auf einer Linie: alle Mittelpunkte auf der Mittellinie der Leiste
  const mid = (b) => b.y + b.height / 2;
  const center = mid(bar);
  for (const [name, b] of [['Maskottchen', mascot], ['Titel', brand], ['Beiträge', posts], ['Themen', cats], ['Tipps und Tricks', tips], ['Suche', search]]) {
    assert.ok(Math.abs(mid(b) - center) <= 2, `${name} vertikal zentriert (Mitte ${mid(b)} statt ${center})`);
  }
  const btn = await box(page, '.topsearch .searchbox button');
  assert.ok(Math.abs(mid(btn) - mid(search)) <= 2, 'Lupe im Suchfeld zentriert');
  assert.equal(await page.locator('.topbar').evaluate((el) => getComputedStyle(el).position), 'sticky');
  await noHScroll(page, 'Desktop');
  await page.close();
});

await test('Topbar bleibt beim Scrollen oben', async () => {
  const page = await open('/2020/10/kurbis/', 1280, 700);
  await page.evaluate(() => scrollTo(0, 1200));
  await page.waitForTimeout(200);
  assert.ok((await box(page, '.topbar')).y >= -1 && (await box(page, '.topbar')).y < 5);
  await page.close();
});

await test('Maskottchen: beide Bilder gleich groß, Mouse-over verschiebt nichts', async () => {
  const page = await open('/', 1280);
  const [n, s] = [await box(page, '.mascot-normal'), await box(page, '.mascot-surprised')];
  assert.deepEqual([n.x, n.y, n.width, n.height], [s.x, s.y, s.width, s.height], 'gleiche Maße und Lage');
  const sels = ['.brand-name', '.topnav a[href="/beitraege/"]', '.topsearch'];
  const before = [];
  for (const sel of sels) before.push(await box(page, sel));
  const op = (sel) => page.locator(sel).evaluate((el) => getComputedStyle(el).opacity);
  assert.equal(await op('.mascot-normal'), '1');
  assert.equal(await op('.mascot-surprised'), '0');
  await page.hover('.brand');
  await page.waitForTimeout(300);
  assert.equal(await op('.mascot-normal'), '0');
  assert.equal(await op('.mascot-surprised'), '1');
  for (const [i, sel] of sels.entries()) assert.deepEqual(await box(page, sel), before[i], sel);
  assert.equal(await page.locator('.mascot img').evaluateAll((els) => els.every((e) => e.alt === '')), true, 'dekorativ');
  await page.mouse.move(700, 600);
  await page.keyboard.press('Tab');
  await page.focus('.brand');
  await page.keyboard.press('Shift+Tab');
  await page.keyboard.press('Tab');
  await page.waitForTimeout(300);
  assert.equal(await op('.mascot-surprised'), '1', 'Tastaturfokus zeigt überrascht');
  await page.close();
});

await test('Titel führt zur Startseite', async () => {
  const page = await open('/2020/10/kurbis/');
  await page.click('.brand');
  await page.waitForURL(BASE + '/');
  await page.close();
});

await test('Beiträge ist ein einfacher Link auf /beitraege/ ohne Dropdown', async () => {
  const page = await open('/', 1280);
  await page.hover('.topnav a[href="/beitraege/"]');
  assert.equal(await page.locator('.topnav .submenu:visible').count(), 0);
  assert.equal(await page.locator('.topnav a[href="/beitraege/"]').locator('xpath=..').locator('.submenu-toggle').count(), 0);
  await page.click('.topnav a[href="/beitraege/"]');
  await page.waitForURL('**/beitraege/');
  await page.close();
});

await test('Themen und Tipps und Tricks sind einfache Links ohne Dropdown und ohne Kategorien', async () => {
  const page = await open('/', 1280);
  assert.equal(await page.locator('.topnav .submenu, .topnav button, .topnav [aria-expanded]').count(), 0, 'keine Dropdowns');
  assert.equal((await page.locator('.topnav').textContent()).includes('Kategori'), false);
  const names = await page.locator('.topnav a').allTextContents();
  assert.deepEqual(names.map((n) => n.trim()), ['Beiträge', 'Themen', 'Tipps und Tricks']);
  await page.hover('.topnav a[href="/themen/"]');
  assert.equal(await page.locator('.topnav li ul:visible').count(), 0);
  await page.click('.topnav a[href="/themen/"]');
  await page.waitForURL('**/themen/');
  await page.goto(BASE + '/');
  await page.click('.topnav a[href="/tipps-und-tricks/"]');
  await page.waitForURL('**/tipps-und-tricks/');
  await page.close();
});

await test('About me steht im Footer und öffnet /about/, Tipps und Tricks nicht im Footer', async () => {
  const page = await open('/', 1280);
  assert.equal(await page.locator('#blogfooter').getByRole('link', { name: 'Tipps und Tricks' }).count(), 0);
  await page.locator('#blogfooter').getByRole('link', { name: 'About me', exact: true }).click();
  await page.waitForURL('**/about/');
  await page.close();
});

await test('Tastatur: Tab-Reihenfolge Marke, Beiträge, Themen, Tipps und Tricks, Suche, Umschalter', async () => {
  const page = await open('/', 1280);
  await page.focus('.brand');
  const seen = [];
  for (let i = 0; i < 6; i++) {
    await page.keyboard.press('Tab');
    seen.push(await page.evaluate(() => document.activeElement.getAttribute('href') ?? document.activeElement.getAttribute('name') ?? document.activeElement.getAttribute('aria-label') ?? document.activeElement.tagName));
  }
  assert.deepEqual(seen.slice(0, 3), ['/beitraege/', '/themen/', '/tipps-und-tricks/']);
  assert.equal(seen[3], 'q');
  const outline = await page.evaluate(() => { const s = getComputedStyle(document.querySelector('.topnav a')); return s.outlineStyle; });
  assert.ok(outline);
  await page.close();
});

await test('Ohne JavaScript: alle Menülinks stehen im Quelltext, nichts klappt auf', async () => {
  const ctx = await browser.newContext({ javaScriptEnabled: false, viewport: { width: 1280, height: 900 } });
  const page = await ctx.newPage();
  await page.goto(BASE + '/');
  for (const href of ['/beitraege/', '/themen/', '/tipps-und-tricks/']) assert.ok(await page.locator(`.topnav a[href="${href}"]`).isVisible(), href);
  await ctx.close();
});

await test('Handy 360 px: Hamburger-Knopf, Liste mit Beiträgen, Themen und Tipps und Tricks, Klickflächen, kein waagerechtes Scrollen', async () => {
  const page = await open('/2020/10/kurbis/', 360, 740, { hasTouch: true, isMobile: true });
  assert.ok(await page.locator('.brand-name').isVisible() && await page.locator('.mascot').isVisible());
  assert.equal(await page.locator('.topnav').isVisible(), false, 'Menü zunächst zu');
  const hb = await box(page, 'details.menu-toggle > summary');
  assert.ok(hb.width >= 44 && hb.height >= 44, 'Hamburger-Knopf mindestens 44 px');
  assert.equal(await page.locator('details.menu-toggle > summary').getAttribute('aria-label'), 'Menü');
  assert.ok(await page.locator('.menu-icon').isVisible(), 'Hamburger-Symbol');
  const [brand, search] = [await box(page, '.brand'), await box(page, '.topsearch')];
  assert.ok(Math.abs((hb.y + hb.height / 2) - (brand.y + brand.height / 2)) <= 3, 'Hamburger und Marke auf einer Linie');
  assert.ok(search.y >= brand.y + brand.height - 1, 'Suchfeld in eigener Zeile');
  await noHScroll(page, 'Handy zu');
  await page.locator('details.menu-toggle > summary').tap();
  assert.ok(await page.locator('.topnav').isVisible());
  for (const name of ['Beiträge', 'Themen', 'Tipps und Tricks']) assert.ok(await page.locator('.topnav > ul > li > a', { hasText: new RegExp(`^${name}$`) }).isVisible(), name);
  assert.equal(await page.locator('.topnav > ul > li > a', { hasText: /About|Kategori/ }).count(), 0);
  const heights = await page.locator('.topnav a').evaluateAll((els) => els.map((e) => e.getBoundingClientRect().height));
  assert.ok(Math.min(...heights) >= 40, `Klickflächen (min ${Math.min(...heights)} px)`);
  await noHScroll(page, 'Handy offen');
  await page.screenshot({ path: '/tmp/flausch-topbar-mobile.png' });
  await page.locator('details.menu-toggle > summary').tap();
  assert.equal(await page.locator('.topnav').isVisible(), false, 'zweiter Tipp klappt das Menü zu');
  await page.close();
});

await test('Handy ohne JavaScript: Menü über details bedienbar', async () => {
  const ctx = await browser.newContext({ javaScriptEnabled: false, viewport: { width: 360, height: 740 }, hasTouch: true, isMobile: true });
  const page = await ctx.newPage();
  await page.goto(BASE + '/');
  assert.ok(await page.locator('details.menu-toggle > summary').isVisible());
  await page.locator('details.menu-toggle > summary').click();
  assert.equal(await page.locator('.topnav').isVisible(), false, 'ohne JS zugeklappt per details');
  await page.locator('details.menu-toggle > summary').click();
  assert.ok(await page.locator('.topnav').isVisible());
  await ctx.close();
});

await test('Tablet 770 px: Topbar in einer Zeile, kein waagerechtes Scrollen', async () => {
  const page = await open('/', 770, 900);
  assert.ok((await box(page, '.topbar')).height < 90, 'eine Zeile');
  await noHScroll(page, 'Tablet');
  await page.close();
});

await test('Archiv /beitraege/: zehn Beiträge, Zustand beim Laden, Ein-/Ausklappen, Alle aufklappen', async () => {
  const page = await open('/beitraege/', 1440);
  assert.equal(await page.locator('h1').textContent(), 'Beiträge');
  assert.equal(await page.locator('main .entry').count(), 10);
  assert.equal(await page.locator('.pagination').count(), 0);
  const [content, archive] = [await box(page, 'main'), await box(page, 'aside.archive')];
  assert.ok(content.x + content.width <= archive.x + 1, 'Archiv rechts');
  const openCount = () => page.locator('.archive details[open]').count();
  assert.equal(await openCount(), 2, 'neuestes Jahr und Monat offen');
  const total = await page.locator('.archive details.month li').count();
  const posts = await (await fetch(BASE + '/sitemap.xml')).text();
  assert.equal(total, (posts.match(/<loc>https:\/\/flauschiversum\.de\/20\d\d\/\d\d\//g) ?? []).length, 'Zahl der Archiv-Einträge gleich Zahl der Beiträge');
  const y2018 = page.locator('.archive details.year', { hasText: /^\s*2018/ }).first();
  await y2018.locator(':scope > summary').click();
  assert.equal(await y2018.getAttribute('open'), '');
  await y2018.locator(':scope > summary').click();
  assert.equal(await y2018.getAttribute('open'), null);
  await page.getByRole('button', { name: 'Alle aufklappen' }).click();
  assert.equal(await page.locator('.archive details:not([open])').count(), 0);
  assert.ok(await page.getByRole('button', { name: 'Alle zuklappen' }).isVisible());
  await page.getByRole('button', { name: 'Alle zuklappen' }).click();
  assert.equal(await openCount(), 0);
  // Tastatur
  await page.locator('.archive details.year > summary').first().focus();
  await page.keyboard.press('Enter');
  assert.equal(await openCount(), 1);
  // Beitrag anklicken
  await page.locator('.archive details.year').first().locator('details.month > summary').first().click();
  await page.locator('.archive details.month a').first().click();
  await page.waitForURL(/\/20\d\d\/\d\d\//);
  await page.close();
});

await test('Archiv ohne JavaScript bedienbar, ohne Knopf', async () => {
  const ctx = await browser.newContext({ javaScriptEnabled: false, viewport: { width: 1280, height: 900 } });
  const page = await ctx.newPage();
  await page.goto(BASE + '/beitraege/');
  assert.equal(await page.locator('.archive-tools').isVisible(), false);
  await page.locator('.archive details.year:not([open]) > summary').first().click();
  assert.ok(await page.locator('.archive details.year[open]').count() >= 2);
  await ctx.close();
});

await test('Archiv am Handy unter der Liste', async () => {
  const page = await open('/beitraege/', 360, 740, { hasTouch: true, isMobile: true });
  const [list, archive] = [await box(page, 'main'), await box(page, 'aside.archive')];
  assert.ok(list.y + list.height <= archive.y + 1);
  await noHScroll(page, 'Archiv Handy');
  await page.close();
});

await test('Themenseite: Beiträge und Archiv nur dieses Themas, jeder Beitrag im Archiv, Handy: Archiv vor den Beiträgen', async () => {
  const page = await open('/themen/', 1440);
  const hrefs = (await page.locator('main ul.taglist a').evaluateAll((els) => els.map((e) => e.getAttribute('href')))).filter((h) => ['/tag/wolle/', '/tag/halloween/', '/tag/filzen/'].includes(h));
  await page.close();
  assert.equal(hrefs.length, 3, 'große und kleine Themen stehen in der Übersicht');
  for (const href of hrefs) {
    const p = await open(href, 1440);
    const listed = await p.locator('main .entry').count();
    assert.ok(listed >= 3, `${href}: Beiträge`);
    assert.equal(await p.locator('.archive details.month li').count(), listed, `${href}: Archiv = Beiträge des Themas`);
    assert.equal(await p.locator('.topic-head .motto, .topic-head .intro').count(), 0, 'kein Reim, keine Kategorie-Einleitung');
    const [content, archive] = [await box(p, 'main'), await box(p, 'aside.archive')];
    assert.ok(content.x + content.width <= archive.x + 1, `${href}: Archiv rechts`);
    await p.close();
  }
  const small = await open(hrefs[0], 360, 740, { hasTouch: true, isMobile: true });
  const [head, archive, list] = [await box(small, '.topic-head'), await box(small, 'aside.archive'), await box(small, 'main')];
  assert.ok(head.y + head.height <= archive.y + 1 && archive.y + archive.height <= list.y + 1, 'Handy: Titel, Archiv, Beiträge');
  await noHScroll(small, 'Themenseite Handy');
  await small.close();
});

await test('Alte Kategorie-Adressen leiten auf die Themen weiter', async () => {
  for (const [from, to] of [['/kategorie/basteln/', '/tag/basteln/'], ['/kategorie/brandings/', '/tag/brandings/'], ['/kategorie/material/', '/tag/einkaufen/'], ['/kategorie/naehen/', '/tag/naehen/']]) {
    const page = await open(from, 1280);
    await page.waitForURL(`**${to}`);
    assert.ok((await page.locator('main .entry').count()) > 3, to);
    await page.close();
  }
});

await test('Blätterseiten /page/<N>/ bleiben erreichbar und sind nicht verlinkt', async () => {
  const page = await open('/page/5/', 1280);
  assert.ok(await page.locator('.pagination').count() > 0);
  assert.equal(await page.locator('main article').count(), 4);
  await page.goto(BASE + '/beitraege/');
  assert.equal(await page.locator('a[href^="/page/"]').count(), 0, 'nicht verlinkt');
  await page.close();
});

await test('Cloud nur auf der Startseite', async () => {
  for (const [path, expected] of [['/', 1], ['/beitraege/', 0], ['/tag/wolle/', 0], ['/2020/10/kurbis/', 0], ['/themen/', 0]]) {
    const page = await open(path, 1280);
    assert.equal(await page.locator('.cloud').count(), expected, path);
    await page.close();
  }
});

await test('Screenshots für die Abnahme', async () => {
  for (const [path, w, name] of [['/', 1440, 'home'], ['/beitraege/', 1440, 'beitraege'], ['/tag/wolle/', 1440, 'thema'], ['/', 390, 'home-mobil']]) {
    const page = await open(path, w, 900);
    await page.screenshot({ path: `/tmp/flausch-${name}.png` });
    await page.close();
  }
});

await browser.close();
for (const r of results) console.log(r.join(' – '));
process.exit(results.every((r) => r[0] === 'ok') ? 0 : 1);
