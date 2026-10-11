// Browser-Tests für Startseite, Themenseiten und Suche.
// Voraussetzung: `flauschiversum serve --port 8099` läuft im Repository,
// Playwright ist installiert (`npm i playwright-core`), Chromium vorhanden.
// Aufruf: node tests/browser/navigation.mjs [Pfad-zu-chromium]
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

const resultTitles = (page) => page.locator('.pagefind-ui__result-title a').allTextContents();
const waitResults = (page) => page.waitForSelector('.pagefind-ui__result', { timeout: 10000 });

await test('Startseite: Überschrift, Begrüßung, zehn Beiträge – in dieser Reihenfolge, Cloud rechts', async () => {
  const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
  await page.goto(BASE + '/');
  const y = (sel) => page.locator(sel).first().evaluate((el) => el.getBoundingClientRect().top + scrollY);
  const ys = [await y('.home-head h1'), await y('.home-head .welcome'), await y('.news .entries')];
  assert.deepEqual(ys, [...ys].sort((a, b) => a - b));
  assert.equal(await page.locator('.home-head h1').textContent(), 'Flauschiversum');
  assert.equal(await page.locator('ul.tiles').count(), 0, 'keine Kategoriekacheln');
  assert.equal(await page.locator('.news .entry').count(), 10);
  assert.equal(await page.locator('main input[type=search]').count(), 0, 'kein Suchfeld im Inhalt');
  assert.ok((await page.locator('.cloud-card .cloud li').count()) > 20, 'Tag-Cloud in der Karte');
  assert.equal(await page.locator('.pagination').count(), 0);
  await page.screenshot({ path: '/tmp/flausch-home.png' });
  await page.close();
});

await test('Kategorie aus dem Menü führt auf die Kategorieseite, Schlagwort auf die Schlagwortseite', async () => {
  const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
  await page.goto(BASE + '/');
  await page.hover('.topnav li.has-menu >> nth=1');
  const first = page.locator('#menu-kategorien a').first();
  const name = (await first.textContent()).trim();
  await first.click();
  await page.waitForURL('**/kategorie/**');
  assert.equal(await page.locator('h1').textContent(), name);
  assert.ok((await page.locator('main .entries .entry').count()) > 3, 'alle Beiträge auf einer Seite');
  await page.goto(BASE + '/');
  await page.locator('.cloud-card .cloud a').first().click();
  await page.waitForURL('**/tag/**');
  assert.ok((await page.locator('.entries .entry').count()) > 0);
  await page.close();
});

await test('Beitrag zeigt Einordnung, Link führt auf die Kategorie', async () => {
  const page = await browser.newPage();
  await page.goto(BASE + '/2020/10/kurbis/');
  const cat = page.locator('.classification a.category');
  assert.ok((await cat.count()) === 1);
  const href = await cat.getAttribute('href');
  await cat.click();
  await page.waitForURL('**' + href);
  await page.close();
});

await test('Suche von der Startseite: Eingabe öffnet /suche/?q= mit Treffern', async () => {
  const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
  await page.goto(BASE + '/');
  await page.fill('.topsearch input[name=q]', 'Stirnband');
  await page.press('.topsearch input[name=q]', 'Enter');
  await page.waitForURL('**/suche/?q=Stirnband');
  await waitResults(page);
  assert.ok((await resultTitles(page)).length > 0);
  await page.close();
});

await test('Suche aus einem Beitrag: Umlaut in der Adresse', async () => {
  const page = await browser.newPage();
  await page.goto(BASE + '/2020/10/kurbis/');
  await page.fill('.topsearch input[name=q]', 'Mütze');
  await page.press('.topsearch input[name=q]', 'Enter');
  await page.waitForURL('**/suche/?q=M%C3%BCtze');
  await waitResults(page);
  await page.close();
});

for (const [query, expect] of [
  ['katze', /katze/i], // Wortform: „Katzen“
  ['kuerbis', /rbis/i], // Umschreibung des Umlauts
  ['kürbis', /rbis/i],
  ['filz', /filz/i], // Wortanfang
]) {
  await test(`Suche „${query}“ findet Treffer mit hervorgehobenem Suchwort`, async () => {
    const page = await browser.newPage();
    await page.goto(`${BASE}/suche/?q=${encodeURIComponent(query)}`);
    await waitResults(page);
    const titles = await resultTitles(page);
    assert.ok(titles.length > 0);
    assert.ok((await page.locator('.pagefind-ui__result mark').count()) > 0, 'Hervorhebung');
    await page.waitForFunction(() => /Ergebnis/.test(document.querySelector('.pagefind-ui__message')?.textContent ?? ''));
    assert.match(await page.locator('.pagefind-ui__message').textContent(), /\d+ Ergebnisse? für/);
    if (query.startsWith('k')) assert.ok(titles.concat(await page.locator('.pagefind-ui__result-excerpt').allTextContents()).some((t) => expect.test(t)));
    await page.close();
  });
}

await test('Suche: alle Dateien von der eigenen Domain', async () => {
  const page = await browser.newPage();
  const hosts = new Set();
  page.on('request', (r) => hosts.add(new URL(r.url()).host));
  await page.goto(`${BASE}/suche/?q=katze`);
  await waitResults(page);
  assert.deepEqual([...hosts], [new URL(BASE).host]);
  await page.close();
});

await test('Suche: Kategoriefilter schränkt die Treffer ein', async () => {
  const page = await browser.newPage();
  await page.goto(`${BASE}/suche/?q=m%C3%BCtze`);
  await waitResults(page);
  await page.waitForFunction(() => /Ergebnis/.test(document.querySelector('.pagefind-ui__message')?.textContent ?? ''));
  const total = await page.locator('.pagefind-ui__message').textContent();
  const n = Number(/(\d+)\s+Ergebnis/.exec(total)?.[1]);
  assert.ok(n > 0, total);
  await page.locator('.pagefind-ui__filter-label', { hasText: 'Wolle' }).click();
  assert.ok(await page.locator('#Kategorie-Wolle').isChecked());
  await page.waitForFunction((n) => {
    const m = /(\d+)\s+Ergebnis/.exec(document.querySelector('.pagefind-ui__message')?.textContent ?? '');
    return m && Number(m[1]) < n;
  }, n);
  await page.close();
});

await test('Suche: Adresse folgt der Eingabe', async () => {
  const page = await browser.newPage();
  await page.goto(`${BASE}/suche/`);
  await page.locator('.pagefind-ui__search-input').fill('hut');
  await page.waitForURL('**/suche/?q=hut');
  await page.close();
});

await test('Suche: keine Treffer verweisen auf Themen', async () => {
  const page = await browser.newPage();
  await page.goto(`${BASE}/suche/?q=qqqzzzxxx`);
  await page.waitForSelector('.pagefind-ui__message');
  assert.match(await page.locator('.pagefind-ui__message').textContent(), /Keine Ergebnisse|keine/i);
  assert.ok(await page.locator('a[href="/themen/"]').count());
  await page.close();
});

await test('Ohne JavaScript: Hinweis, Formular und Links zum Stöbern', async () => {
  const ctx = await browser.newContext({ javaScriptEnabled: false });
  const page = await ctx.newPage();
  await page.goto(`${BASE}/suche/?q=katze`);
  assert.match(await page.locator('noscript').first().textContent(), /JavaScript/);
  assert.ok(await page.locator('#search-form input[name=q]').isVisible());
  assert.ok(await page.locator('main a[href="/themen/"]').first().isVisible());
  assert.ok(await page.locator('main a[href="/beitraege/"]').first().isVisible());
  await ctx.close();
});

for (const path of ['/', '/themen/', '/kategorie/basteln/', '/tag/katzen/', '/suche/?q=katze', '/2020/10/kurbis/', '/beitraege/']) {
  await test(`Handy ${path}: kein waagerechtes Scrollen`, async () => {
    const page = await browser.newPage({ viewport: { width: 360, height: 740 }, hasTouch: true, isMobile: true });
    await page.goto(BASE + path);
    await page.waitForTimeout(500);
    const [sw, cw] = await page.evaluate(() => [document.documentElement.scrollWidth, document.documentElement.clientWidth]);
    assert.ok(sw <= cw, `scrollWidth ${sw} > clientWidth ${cw}`);
    if (path === '/') await page.screenshot({ path: '/tmp/flausch-home-mobile.png', fullPage: true });
    await page.close();
  });
}

await browser.close();
for (const r of results) console.log(r.join(' – '));
process.exit(results.every((r) => r[0] === 'ok') ? 0 : 1);
