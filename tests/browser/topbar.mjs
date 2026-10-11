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

await test('Topbar am Desktop: Maskottchen und Titel links, About me, Beiträge, Kategorien rechts, dann Suche', async () => {
  const page = await open('/2020/10/kurbis/', 1280);
  const [mascot, brand, about, posts, cats, search] = [await box(page, '.mascot'), await box(page, '.brand-name'), await box(page, '.topnav a[href="/about/"]'), await box(page, '.topnav a[href="/beitraege/"]'), await box(page, '.topnav a[href="/themen/"]'), await box(page, '.topsearch')];
  assert.ok(mascot.x < brand.x && brand.x + brand.width < about.x, 'Maskottchen, Titel, dann Menüs');
  assert.ok(about.x < posts.x && posts.x < cats.x && cats.x + cats.width <= search.x + 1, 'Reihenfolge der Menüs, Suche zuletzt');
  assert.ok(mascot.x < 200, 'ganz links');
  assert.ok(search.x + search.width > 1280 - 100 && about.x > 300, 'Menüs rechtsbündig');
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
  const sels = ['.brand-name', '.topnav a[href="/about/"]', '.topsearch'];
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

await test('Menü Beiträge: zehn neueste und „Alle Beiträge“; Klick auf Beiträge öffnet /beitraege/', async () => {
  const page = await open('/', 1280);
  await page.hover('.topnav li.has-menu >> nth=0');
  assert.ok(await page.locator('#menu-beitraege').isVisible());
  const links = await page.locator('#menu-beitraege a').evaluateAll((els) => els.map((e) => [e.textContent.trim(), e.getAttribute('href')]));
  assert.equal(links.length, 11);
  assert.deepEqual(links.at(-1), ['Alle Beiträge', '/beitraege/']);
  const homeTitles = await page.locator('.news .entry-title a').allTextContents();
  assert.deepEqual(links.slice(0, 10).map((l) => l[0]), homeTitles, 'gleiche zehn Beiträge wie auf der Startseite');
  await page.screenshot({ path: '/tmp/flausch-menu.png' });
  await page.click('.topnav a[href="/beitraege/"]');
  await page.waitForURL('**/beitraege/');
  await page.close();
});

await test('Menü Kategorien: alle Kategorien, Klick auf Kategorien öffnet /themen/', async () => {
  const page = await open('/themen/', 1280);
  const expected = await page.locator('main section:first-of-type ul.taglist a').evaluateAll((els) => els.map((e) => e.getAttribute('href')));
  await page.hover('.topnav li.has-menu >> nth=1');
  assert.ok(await page.locator('#menu-kategorien').isVisible());
  const hrefs = await page.locator('#menu-kategorien a').evaluateAll((els) => els.map((e) => e.getAttribute('href')));
  assert.deepEqual(hrefs, expected, 'dieselben Kategorien wie auf /themen/');
  await page.close();
  const p2 = await open('/', 1280);
  await p2.click('.topnav a[href="/themen/"]');
  await p2.waitForURL('**/themen/');
  await p2.close();
});

await test('About me: einfacher Link ohne Untermenü', async () => {
  const page = await open('/', 1280);
  await page.hover('.topnav a[href="/about/"]');
  assert.equal(await page.locator('.topnav .submenu:visible').count(), 0);
  await page.click('.topnav a[href="/about/"]');
  await page.waitForURL('**/about/');
  await page.close();
});

await test('Tastatur: Knopf öffnet, aria-expanded, Pfeile, Escape; nur ein Menü offen', async () => {
  const page = await open('/', 1280);
  const btn = (i) => page.locator('.submenu-toggle').nth(i);
  await btn(1).focus();
  assert.equal(await btn(1).getAttribute('aria-expanded'), 'false');
  await page.keyboard.press('Enter');
  assert.equal(await btn(1).getAttribute('aria-expanded'), 'true');
  assert.ok(await page.locator('#menu-kategorien').isVisible());
  await page.keyboard.press('ArrowDown');
  assert.equal(await page.evaluate(() => document.activeElement.closest('#menu-kategorien') !== null), true, 'Fokus im Untermenü');
  await page.keyboard.press('Escape');
  assert.equal(await btn(1).getAttribute('aria-expanded'), 'false');
  assert.equal(await page.locator('#menu-kategorien').isVisible(), false);
  assert.equal(await page.evaluate(() => document.activeElement.classList.contains('submenu-toggle')), true, 'Fokus zurück auf dem Knopf');
  // nur ein Menü offen
  await btn(0).click();
  assert.equal(await btn(0).getAttribute('aria-expanded'), 'true');
  await btn(1).click();
  assert.equal(await btn(0).getAttribute('aria-expanded'), 'false');
  assert.equal(await btn(1).getAttribute('aria-expanded'), 'true');
  // Klick außerhalb schließt
  await page.mouse.click(300, 600);
  assert.equal(await btn(1).getAttribute('aria-expanded'), 'false');
  await page.close();
});

await test('Ohne JavaScript: Hover öffnet das Untermenü, Links stehen im Quelltext', async () => {
  const ctx = await browser.newContext({ javaScriptEnabled: false, viewport: { width: 1280, height: 900 } });
  const page = await ctx.newPage();
  await page.goto(BASE + '/');
  assert.equal(await page.locator('#menu-beitraege').isVisible(), false);
  await page.hover('.topnav li.has-menu >> nth=0');
  assert.ok(await page.locator('#menu-beitraege').isVisible());
  assert.equal(await page.locator('#menu-beitraege a').count(), 11);
  await page.hover('.topnav li.has-menu >> nth=1');
  assert.ok(await page.locator('#menu-kategorien').isVisible());
  await ctx.close();
});

await test('Handy 360 px: Menü-Knopf, Liste mit allen Unterpunkten, Klickflächen, kein waagerechtes Scrollen', async () => {
  const page = await open('/2020/10/kurbis/', 360, 740, { hasTouch: true, isMobile: true });
  assert.ok(await page.locator('.brand-name').isVisible() && await page.locator('.mascot').isVisible());
  assert.equal(await page.locator('.topnav').isVisible(), false, 'Menü zunächst zu');
  await noHScroll(page, 'Handy zu');
  await page.locator('details.menu-toggle > summary').tap();
  assert.ok(await page.locator('.topnav').isVisible());
  for (const name of ['About me', 'Beiträge', 'Kategorien']) assert.ok(await page.locator('.topnav > ul > li > a', { hasText: new RegExp(`^${name}$`) }).isVisible(), name);
  assert.ok(await page.locator('#menu-beitraege a').first().isVisible() && await page.locator('#menu-kategorien a').first().isVisible(), 'Unterpunkte sichtbar');
  const heights = await page.locator('.topnav a').evaluateAll((els) => els.map((e) => e.getBoundingClientRect().height));
  assert.ok(Math.min(...heights) >= 40, `Klickflächen (min ${Math.min(...heights)} px)`);
  assert.ok((await box(page, 'details.menu-toggle > summary')).height >= 44);
  await noHScroll(page, 'Handy offen');
  await page.screenshot({ path: '/tmp/flausch-topbar-mobile.png' });
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

await test('Kategorieseite: Text vor den Beiträgen, Archiv nur dieser Kategorie, jeder Beitrag im Archiv', async () => {
  const page = await open('/themen/', 1440);
  const hrefs = await page.locator('main section:first-of-type ul.taglist a').evaluateAll((els) => els.map((e) => e.getAttribute('href')));
  await page.close();
  assert.ok(hrefs.length >= 1);
  for (const href of hrefs) {
    const p = await open(href, 1440);
    const intro = await box(p, '.topic-head .intro');
    const firstEntry = await box(p, 'main .entry');
    assert.ok(intro && intro.y + intro.height <= firstEntry.y, `${href}: Einleitungstext vor den Beiträgen`);
    const listed = await p.locator('main .entry').count();
    assert.equal(await p.locator('.archive details.month li').count(), listed, `${href}: Archiv = Beiträge der Kategorie`);
    const [content, archive] = [await box(p, 'main'), await box(p, 'aside.archive')];
    assert.ok(content.x + content.width <= archive.x + 1, `${href}: Archiv rechts`);
    await p.close();
  }
  const small = await open(hrefs[0], 360, 740, { hasTouch: true, isMobile: true });
  const [head, archive, list] = [await box(small, '.topic-head'), await box(small, 'aside.archive'), await box(small, 'main')];
  assert.ok(head.y + head.height <= archive.y + 1 && archive.y + archive.height <= list.y + 1, 'Handy: Text, Archiv, Beiträge');
  await noHScroll(small, 'Kategorie Handy');
  await small.close();
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
  for (const [path, expected] of [['/', 1], ['/beitraege/', 0], ['/kategorie/wolle/', 0], ['/2020/10/kurbis/', 0], ['/themen/', 0]]) {
    const page = await open(path, 1280);
    assert.equal(await page.locator('.cloud').count(), expected, path);
    await page.close();
  }
});

await test('Screenshots für die Abnahme', async () => {
  for (const [path, w, name] of [['/', 1440, 'home'], ['/beitraege/', 1440, 'beitraege'], ['/kategorie/wolle/', 1440, 'kategorie'], ['/', 390, 'home-mobil']]) {
    const page = await open(path, w, 900);
    await page.screenshot({ path: `/tmp/flausch-${name}.png` });
    await page.close();
  }
});

await browser.close();
for (const r of results) console.log(r.join(' – '));
process.exit(results.every((r) => r[0] === 'ok') ? 0 : 1);
