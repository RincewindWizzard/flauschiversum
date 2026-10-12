// Browser-Tests für Seitenaufbau (Topbar, Karten), Footer, Autor, Suchfeld, Typografie und Farben.
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

await test('1440 px: Beitragsseite einspaltig, Text mittig, keine Seitenspalten', async () => {
  const page = await open(POST, 1440);
  const content = await box(page, 'main.page');
  const fs = await page.locator('main.page').evaluate((el) => parseFloat(getComputedStyle(el).fontSize));
  assert.ok(content.width / fs <= 40, `Lesetext höchstens etwa 40 em (${content.width / fs} em)`);
  assert.ok(Math.abs(content.x + content.width / 2 - 720) < 30, 'Text mittig');
  assert.equal(await visibleCount(page, 'aside:not(.related)'), 0, 'keine Seitenspalten');
  assert.equal(await visibleCount(page, 'input[type=search]'), 1, 'ein Suchfeld');
  await page.close();
});

await test('1440 px: Startseite mit Cloud-Karte rechts, 360 px darunter, ohne waagerechtes Scrollen', async () => {
  const page = await open('/', 1440);
  const [content, aside] = [await box(page, 'main#home'), await box(page, 'aside.cloud-card')];
  assert.ok(content.x + content.width <= aside.x + 1, 'Karte rechts neben den Beiträgen');
  await page.close();
  const small = await open('/', 360, 740, { hasTouch: true, isMobile: true });
  const [c2, a2, footer] = [await box(small, 'main#home'), await box(small, 'aside.cloud-card'), await box(small, '#blogfooter')];
  assert.ok(c2.y + c2.height <= a2.y + 1 && a2.y + a2.height <= footer.y + 1, 'Reihenfolge Beiträge, Karte, Footer');
  const [sw, cw] = await small.evaluate(() => [document.documentElement.scrollWidth, document.documentElement.clientWidth]);
  assert.ok(sw <= cw, `scrollWidth ${sw} > clientWidth ${cw}`);
  await small.close();
});

await test('Suche und Themenübersicht: einspaltig ohne Karten, Topbar vorhanden', async () => {
  for (const path of ['/suche/?q=katze', '/themen/']) {
    const page = await open(path, 1440);
    assert.equal(await visibleCount(page, 'aside'), 0, path);
    assert.equal(await visibleCount(page, '.topbar .topnav'), 1, path);
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
    for (const name of ['Tipps und Tricks', 'About me', 'Impressum']) {
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

await test('Footer wie die Segelgruppe: dunkles Band, links Icons mit Copyright darunter, rechts Links untereinander', async () => {
  const page = await open(POST, 1280, 800);
  const f = page.locator('#blogfooter');
  const band = await f.boundingBox();
  assert.ok(band.width >= 1279, `Band über die volle Breite (${band.width} px)`);
  const bg = await f.evaluate((el) => getComputedStyle(el).backgroundColor);
  assert.notEqual(bg, 'rgba(0, 0, 0, 0)');
  const [r, g, b] = bg.match(/\d+/g).map(Number);
  assert.ok(r + g + b < 3 * 110, `dunkler Hintergrund ${bg}`);
  const links = await f.locator('.footer-links a').evaluateAll((els) => els.map((e) => { const r = e.getBoundingClientRect(); return [r.x, r.y, r.height]; }));
  assert.equal(links.length, 3);
  assert.ok(links.every((l) => Math.abs(l[0] - links[0][0]) < 1), 'Links stehen in einer Spalte');
  assert.ok(links[0][1] < links[1][1] && links[1][1] < links[2][1], 'untereinander');
  const icons = await f.locator('.socials a').evaluateAll((els) => els.map((e) => { const r = e.getBoundingClientRect(); return [r.x, r.y]; }));
  assert.equal(icons.length, 4);
  assert.ok(icons.every((i) => Math.abs(i[1] - icons[0][1]) < 1) && icons[0][0] < icons[3][0], 'Icons in einer Zeile');
  const [socials, copyright, linkBox] = [await f.locator('.socials').boundingBox(), await f.locator('.copyright').boundingBox(), await f.locator('.footer-links').boundingBox()];
  assert.ok(copyright.y >= socials.y + socials.height - 1, 'Copyright unter den Icons');
  assert.ok(Math.abs(copyright.x - socials.x) < 12, 'Copyright linksbündig unter den Icons');
  assert.ok(linkBox.x > socials.x + socials.width - 1 && linkBox.x > 640, 'Links rechts neben dem linken Block');
  const iconH = await f.locator('.socials .icon').first().evaluate((el) => [el.getBoundingClientRect().height, parseFloat(getComputedStyle(el.closest('#blogfooter')).fontSize)]);
  assert.ok(Math.abs(iconH[0] - 3 * iconH[1]) < 2, `Icons 3 em hoch (${iconH})`);
  assert.equal(await f.evaluate((el) => getComputedStyle(el).color), 'rgb(175, 175, 175)', 'Schriftfarbe #afafaf');
  assert.equal(await f.locator('.footer-links a').first().evaluate((el) => getComputedStyle(el).textDecorationLine), 'none');
  assert.ok(await f.locator('.footer-links a').first().evaluate((el) => parseFloat(getComputedStyle(el).fontSize)) >= 20, 'große Links');
  await f.screenshot({ path: '/tmp/flausch-footer-kompakt.png' });
  await page.close();
});

await test('Footer am Handy: Icons, Links, zuletzt das Copyright', async () => {
  const page = await open(POST, 360, 740, { hasTouch: true, isMobile: true });
  const f = page.locator('#blogfooter');
  const [socials, links, copyright] = [await f.locator('.socials').boundingBox(), await f.locator('.footer-links').boundingBox(), await f.locator('.copyright').boundingBox()];
  assert.ok(socials.y + socials.height <= links.y + 1, 'Icons über den Links');
  assert.ok(links.y + links.height <= copyright.y + 1, 'Copyright unter den Links');
  const docH = await page.evaluate(() => document.documentElement.scrollHeight);
  const ft = await f.boundingBox();
  assert.ok(copyright.y + copyright.height <= ft.y + ft.height + 1 && ft.y + ft.height >= docH - 2, 'Copyright ist das letzte Element der Seite');
  const [sw, cw] = await page.evaluate(() => [document.documentElement.scrollWidth, document.documentElement.clientWidth]);
  assert.ok(sw <= cw, 'kein waagerechtes Scrollen');
  await f.screenshot({ path: '/tmp/flausch-footer-handy.png' });
  await page.close();
});

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
    assert.equal(await footer.getByRole('link', { name: 'Abonnieren', exact: true }).count(), 0, 'kein Textlink „Abonnieren“, das RSS-Icon genügt');
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

await test('Karten: Cloud, Archiv und Untermenü liegen auf hellen, abgerundeten Karten', async () => {
  const home = await open('/', 1440);
  const c = await cardStyle(home, 'aside.cloud-card');
  assert.ok(c.alpha > 0 && c.radius >= 8 && c.pad >= 8, `Cloud: ${JSON.stringify(c)}`);
  await home.hover('.topnav li.has-menu >> nth=0');
  const m = await cardStyle(home, '.topnav .submenu');
  assert.ok(m.alpha > 0 && m.radius >= 8, `Untermenü: ${JSON.stringify(m)}`);
  await home.close();
  const arch = await open('/beitraege/', 1440);
  const a = await cardStyle(arch, 'aside.archive');
  assert.ok(a.alpha > 0 && a.radius >= 8 && a.pad >= 8, `Archiv: ${JSON.stringify(a)}`);
  await arch.close();
});

const searchBoxChecks = async (page, root) => {
  const input = await page.locator(`${root} input[type=search]`).first().boundingBox();
  const button = await page.locator(`${root} .searchbox button`).first().boundingBox();
  assert.ok(button.x >= input.x && button.x + button.width <= input.x + input.width + 1, `Lupe liegt im Feld (${root})`);
  assert.ok(button.y >= input.y && button.y + button.height <= input.y + input.height + 1, `Lupe liegt vertikal im Feld (${root})`);
  assert.ok(input.height < 70, `Feld ist einzeilig (${root})`);
  assert.equal(await page.locator(`${root} .searchbox button`).first().getAttribute('aria-label'), 'Suchen');
};

await test('Suchfeld: Lupe im Feld in der Topbar am Desktop und am Handy, eine Zeile', async () => {
  const page = await open(POST, 1440);
  await searchBoxChecks(page, '.topsearch');
  await page.close();
  const small = await open(POST, 360, 740, { hasTouch: true, isMobile: true });
  await searchBoxChecks(small, '.topsearch');
  await small.close();
  const search = await open('/suche/', 1440);
  await searchBoxChecks(search, '#search-form');
  await search.close();
});

await test('Suchfeld: Lupe und Enter öffnen /suche/?q=', async () => {
  const page = await open(POST, 1440);
  await page.locator('.topsearch input[type=search]').fill('Katze');
  await page.locator('.topsearch .searchbox button').click();
  await page.waitForURL('**/suche/?q=Katze');
  await page.close();
  const p2 = await open('/', 1440);
  await p2.locator('.topsearch input[type=search]').fill('Filz');
  await p2.keyboard.press('Enter');
  await p2.waitForURL('**/suche/?q=Filz');
  await p2.close();
});

await test('Typografie: Navigation und Footer in Textschrift, Logo und Titel in Lobster', async () => {
  const page = await open('/', 1440);
  const font = (sel) => page.locator(sel).first().evaluate((el) => { const s = getComputedStyle(el); return [s.fontFamily, parseFloat(s.fontSize), s.fontWeight]; });
  for (const sel of ['.topnav > ul > li > a', '#blogfooter .footer-links a', '.cloud-card .cloud a']) {
    const [family, size] = await font(sel);
    assert.ok(!/lobster/i.test(family), `${sel}: ${family}`);
    assert.ok(size >= 0.9 * 16, `${sel}: ${size}px`);
  }
  for (const sel of ['.brand-name', '.cloud-card .side-title']) assert.match((await font(sel))[0], /lobster/i, sel);
  assert.ok(Number((await font('.topnav > ul > li > a'))[2]) >= 600, 'Menüs halbfett');
  await page.close();
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

await test('Farben: Hintergrund Lavendelweiß (Flauschi-Nebel), Kontrast von Text, Links, Knopf und Tag mindestens 4,5 : 1', async () => {
  const page = await open(POST, 1440);
  assert.equal(await page.evaluate(() => getComputedStyle(document.body).backgroundColor), 'rgb(246, 243, 252)');
  for (const [sel, name] of [['main.page p', 'Text'], ['main.page a', 'Link im Text'], ['.topnav a', 'Navigation'], ['.searchbox button', 'Lupe'], ['.post-header a.category', 'Kategorie-Tag'], ['.post-header a.tag', 'Schlagwort-Tag'], ['#blogfooter .footer-links a', 'Footer'], ['#blogfooter .copyright', 'Copyright']]) {
    const c = await contrast(page, sel);
    assert.ok(c >= 4.5, `${name}: Kontrast ${c.toFixed(2)}`);
  }
  await page.close();
  const home = await open('/', 1440);
  const cc = await contrast(home, '.cloud a');
  assert.ok(cc >= 4.5, `Cloud: Kontrast ${cc.toFixed(2)}`);
  await home.close();
  const css = await (await fetch(BASE + '/style.css')).text();
  for (const neon of ['#bdf239', '#82e536', '#eefc3c']) assert.ok(!css.toLowerCase().includes(neon), `Neongrün ${neon} im Stylesheet`);
});

await browser.close();
for (const r of results) console.log(r.join(' – '));
process.exit(results.every((r) => r[0] === 'ok') ? 0 : 1);
