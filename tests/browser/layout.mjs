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
  const size = async (name) => page.locator('.cloud a', { hasText: new RegExp(`^${name}\\s*\\d+$`) }).evaluate((el) => parseFloat(getComputedStyle(el).fontSize));
  const base = await page.locator('.cloud').evaluate((el) => parseFloat(getComputedStyle(el).fontSize));
  assert.ok((await size('Hüte')) > (await size('Weihnachten')), 'häufiger = größer');
  const all = await page.locator('.cloud a').evaluateAll((els) => els.map((e) => parseFloat(getComputedStyle(e).fontSize)));
  assert.ok(Math.min(...all) / base >= 0.89 && Math.max(...all) / base <= 1.11, `Größen zwischen 0,9 und 1,1 em (${Math.min(...all) / base}–${Math.max(...all) / base})`);
  const names = (await page.locator('.cloud a').evaluateAll((els) => els.map((e) => e.childNodes[0].textContent.trim())));
  assert.deepEqual(names, [...names].sort((a, b) => a.localeCompare(b, 'de')), 'alphabetisch');
  assert.ok(await page.locator('.cloud a .count').first().isVisible(), 'Zahl der Beiträge sichtbar');
  const colors = await page.locator('.cloud a').evaluateAll((els) => [...new Set(els.map((e) => getComputedStyle(e).color))]);
  assert.equal(colors.length, 1, `einheitliche Textfarbe in der Cloud: ${colors}`);
  await page.locator('.cloud a', { hasText: /^Hüte\s*\d+$/ }).click();
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
      assert.ok(await footer.getByRole('link', { name, exact: true }).isVisible(), name);
    }
    for (const [name, href] of [['Instagram', 'https://www.instagram.com/ermelinewollknoll/'], ['Mastodon', 'https://chaos.social/@ErmelineWollknoll']]) {
      const link = footer.getByRole('link', { name });
      assert.ok(await link.isVisible(), name);
      assert.equal(await link.getAttribute('href'), href);
      const b = await link.boundingBox();
      assert.ok(b.width >= 44 && b.height >= 44, `Klickfläche ${name}: ${b.width}×${b.height}`);
    }
    const mailLink = footer.getByRole('link', { name: 'E-Mail schreiben' });
    assert.ok(await mailLink.isVisible());
    assert.equal(await mailLink.getAttribute('href'), 'mailto:ermeline@flauschiversum.de?subject=Flauschiversum');
    assert.equal(await mailLink.getAttribute('title'), 'ermeline@flauschiversum.de');
    assert.ok(!(await footer.textContent()).includes('@'), 'Adresse nicht als Text');
    const mb = await mailLink.boundingBox();
    assert.ok(mb.width >= 44 && mb.height >= 44, `Klickfläche E-Mail: ${mb.width}×${mb.height}`);
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

for (const [width, label] of [[1280, 'Desktop'], [360, 'Handy']]) {
  await test(`RSS-Icon ${label}: sichtbar nach Mastodon, Klickfläche, Tastatur, öffnet den Feed`, async () => {
    const page = await open(POST, width, 800, width < 500 ? { hasTouch: true, isMobile: true } : {});
    const footer = page.locator('#blogfooter');
    const rss = footer.getByRole('link', { name: 'RSS-Feed abonnieren' });
    assert.ok(await rss.isVisible());
    assert.equal(await rss.getAttribute('href'), '/rss.xml');
    assert.equal(await rss.getAttribute('type'), 'application/rss+xml');
    const [m, r] = [await footer.getByRole('link', { name: 'Mastodon' }).boundingBox(), await rss.boundingBox()];
    assert.ok(r.x > m.x, 'RSS steht nach Mastodon');
    assert.ok(r.width >= 44 && r.height >= 44, `Klickfläche ${r.width}×${r.height}`);
    // Tastatur: von Mastodon ein Tab weiter, Fokusrahmen sichtbar
    await footer.getByRole('link', { name: 'Mastodon' }).focus();
    await page.keyboard.press('Tab');
    assert.equal(await page.evaluate(() => document.activeElement?.getAttribute('aria-label')), 'RSS-Feed abonnieren');
    const outline = await page.evaluate(() => { const s = getComputedStyle(document.activeElement); return [s.outlineStyle, parseFloat(s.outlineWidth)]; });
    assert.ok(outline[0] !== 'none' && outline[1] > 0, `Fokusrahmen ${outline}`);
    const [sw, cw] = await page.evaluate(() => [document.documentElement.scrollWidth, document.documentElement.clientWidth]);
    assert.ok(sw <= cw, 'kein waagerechtes Scrollen');
    const res = await page.request.get(BASE + '/rss.xml');
    assert.equal(res.status(), 200);
    assert.match(await res.text(), /<rss[ >]/);
    // Der Textlink „Abonnieren“ bleibt
    assert.equal(await footer.getByRole('link', { name: 'Abonnieren', exact: true }).getAttribute('href'), '/rss.xml');
    if (width > 500) await footer.screenshot({ path: '/tmp/flausch-footer-rss.png' });
    else await footer.screenshot({ path: '/tmp/flausch-footer-rss-handy.png' });
    await page.close();
  });
}

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

// ---------- Karten, Suchfeld, Typografie, Farben ----------
const cardStyle = (page, sel) =>
  page.locator(sel).first().evaluate((el) => {
    const s = getComputedStyle(el);
    const alpha = (c) => (c.startsWith('rgba') ? parseFloat(c.split(',')[3]) : c === 'rgba(0, 0, 0, 0)' ? 0 : 1);
    return { alpha: alpha(s.backgroundColor), radius: parseFloat(s.borderTopLeftRadius), pad: parseFloat(s.paddingLeft) };
  });

await test('Karten: Navigationsspalte und Sidebar liegen auf hellen, abgerundeten Karten (1440 px)', async () => {
  const page = await open(POST, 1440);
  for (const sel of ['.sitenav', '.sidebar .sidebar-cloud']) {
    const c = await cardStyle(page, sel);
    assert.ok(c.alpha > 0 && c.radius >= 8 && c.pad >= 8, `${sel}: ${JSON.stringify(c)}`);
  }
  await page.close();
});

await test('Karten: Sidebar-Blöcke auf dem Handy, ohne waagerechtes Scrollen', async () => {
  const page = await open(POST, 360, 740, { hasTouch: true, isMobile: true });
  for (const sel of ['.sidebar .sidebar-cats', '.sidebar .sidebar-cloud']) {
    const c = await cardStyle(page, sel);
    assert.ok(c.alpha > 0 && c.radius >= 8, `${sel}: ${JSON.stringify(c)}`);
  }
  const [sw, cw] = await page.evaluate(() => [document.documentElement.scrollWidth, document.documentElement.clientWidth]);
  assert.ok(sw <= cw);
  await page.close();
});

const searchBoxChecks = async (page, root) => {
  const input = await page.locator(`${root} input[type=search]`).first().boundingBox();
  const button = await page.locator(`${root} .searchbox button`).first().boundingBox();
  assert.ok(button.x >= input.x && button.x + button.width <= input.x + input.width + 1, `Lupe liegt im Feld (${root})`);
  assert.ok(button.y >= input.y && button.y + button.height <= input.y + input.height + 1, `Lupe liegt vertikal im Feld (${root})`);
  assert.ok(input.height < 70, `Feld ist einzeilig (${root})`);
  assert.equal(await page.locator(`${root} .searchbox button`).first().getAttribute('aria-label'), 'Suchen');
};

await test('Suchfeld: Lupe im Feld in der 13-em-Spalte und im Kopf, ein Zeile', async () => {
  const page = await open(POST, 1440);
  await searchBoxChecks(page, '.sitenav');
  await page.close();
  const small = await open(POST, 360, 740, { hasTouch: true, isMobile: true });
  await searchBoxChecks(small, '.topsearch');
  await small.close();
  const home = await open('/', 1440);
  await searchBoxChecks(home, '.home-search');
  await home.close();
});

await test('Suchfeld: Lupe und Enter öffnen /suche/?q=', async () => {
  const page = await open(POST, 1440);
  await page.locator('.sitenav input[type=search]').fill('Katze');
  await page.locator('.sitenav .searchbox button').click();
  await page.waitForURL('**/suche/?q=Katze');
  await page.close();
  const p2 = await open('/', 1440);
  await p2.locator('.home-search input[type=search]').fill('Filz');
  await p2.keyboard.press('Enter');
  await p2.waitForURL('**/suche/?q=Filz');
  await p2.close();
});

await test('Typografie: Navigation und Footer in Textschrift, Logo und Titel in Lobster', async () => {
  const page = await open('/', 1440);
  const font = (sel) => page.locator(sel).first().evaluate((el) => { const s = getComputedStyle(el); return [s.fontFamily, parseFloat(s.fontSize), s.fontWeight]; });
  for (const sel of ['.sitenav .sitenav-links a', '#blogfooter .footer-links a', '.sidebar .cloud a', '.sitenav .cats a']) {
    const [family, size] = await font(sel);
    assert.ok(!/lobster/i.test(family), `${sel}: ${family}`);
    assert.ok(size >= 0.9 * 16, `${sel}: ${size}px`);
  }
  for (const sel of ['#blogheader .logo h1', '#kategorien', '#neu']) assert.match((await font(sel))[0], /lobster/i, sel);
  assert.ok(Number((await font('#blogfooter .footer-links a'))[2]) >= 600, 'Footer-Links halbfett');
  await page.close();
  const mid = await open(POST, 1100);
  const fam = await mid.locator('#blogheader .topnav a').first().evaluate((el) => getComputedStyle(el).fontFamily);
  assert.ok(!/lobster/i.test(fam), `Kopfnavigation: ${fam}`);
  await mid.close();
});

// Kontrast nach WCAG
const contrast = (page, sel, bgSel) =>
  page.evaluate(([sel, bgSel]) => {
    const parse = (c) => c.match(/[\d.]+/g).map(Number);
    const lum = ([r, g, b]) => [r, g, b].map((v) => { v /= 255; return v <= 0.03928 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4; }).reduce((a, v, i) => a + v * [0.2126, 0.7152, 0.0722][i], 0);
    const bgOf = (el) => { for (let e = el; e; e = e.parentElement) { const c = parse(getComputedStyle(e).backgroundColor); if ((c[3] ?? 1) > 0.9) return c.slice(0, 3); } return [255, 255, 255]; };
    const el = document.querySelector(sel);
    const fg = parse(getComputedStyle(el).color).slice(0, 3);
    const bg = bgSel ? bgOf(document.querySelector(bgSel)) : bgOf(el);
    const [a, b] = [lum(fg), lum(bg)].sort((x, y) => y - x);
    return (a + 0.05) / (b + 0.05);
  }, [sel, bgSel]);

await test('Farben: Hintergrund Cremeweiß, Kontrast von Text, Links, Knopf und Tag mindestens 4,5 : 1', async () => {
  const page = await open(POST, 1440);
  assert.equal(await page.evaluate(() => getComputedStyle(document.body).backgroundColor), 'rgb(250, 247, 240)');
  for (const [sel, name] of [['main.page p', 'Text'], ['main.page a', 'Link im Text'], ['.sidebar .cloud a', 'Cloud'], ['.sitenav .sitenav-links a', 'Navigation'], ['.searchbox button', 'Lupe'], ['.post-header a.category', 'Kategorie-Tag'], ['.post-header a.tag', 'Schlagwort-Tag'], ['#blogfooter .footer-links a', 'Footer'], ['#blogfooter .copyright', 'Copyright']]) {
    const c = await contrast(page, sel);
    assert.ok(c >= 4.5, `${name}: Kontrast ${c.toFixed(2)}`);
  }
  await page.close();
  const css = await (await fetch(BASE + '/style.css')).text();
  for (const neon of ['#bdf239', '#82e536', '#eefc3c']) assert.ok(!css.toLowerCase().includes(neon), `Neongrün ${neon} im Stylesheet`);
});

await browser.close();
for (const r of results) console.log(r.join(' – '));
process.exit(results.every((r) => r[0] === 'ok') ? 0 : 1);
