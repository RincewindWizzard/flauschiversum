// Browser-Tests für Farbschema, Dunkelmodus und Umschalter.
// Voraussetzung: `flauschiversum serve --port 8099` läuft im Repository,
// Playwright ist installiert (`npm i playwright-core`), Chromium vorhanden.
// Aufruf: node tests/browser/theme.mjs [Pfad-zu-chromium]
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
const newPage = async (colorScheme, path, extra = {}) => {
  const ctx = await browser.newContext({ colorScheme, viewport: { width: 1440, height: 900 }, ...extra });
  const page = await ctx.newPage();
  await page.goto(BASE + path);
  return page;
};

// ---------- Farbrechnung (WCAG, Machado für Farbfehlsichtigkeit) ----------
const lin = (c) => { c /= 255; return c <= 0.03928 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4; };
const lum = ([r, g, b]) => 0.2126 * lin(r) + 0.7152 * lin(g) + 0.0722 * lin(b);
const ratio = (a, b) => { const [x, y] = [lum(a), lum(b)].sort((p, q) => q - p); return (x + 0.05) / (y + 0.05); };
const MACHADO = {
  Protanopie: [[0.152286, 1.052583, -0.204868], [0.114503, 0.786281, 0.099216], [-0.003882, -0.048116, 1.051998]],
  Deuteranopie: [[0.367322, 0.860646, -0.227968], [0.280085, 0.672501, 0.047413], [-0.01182, 0.04294, 0.968881]],
  Tritanopie: [[1.255528, -0.076749, -0.178779], [-0.078411, 0.930809, 0.147602], [0.004733, 0.691367, 0.3039]],
};
const gamma = (c) => 255 * (c <= 0.0031308 ? 12.92 * c : 1.055 * c ** (1 / 2.4) - 0.055);
const simulate = (rgb, m) => {
  const v = rgb.map(lin);
  return m.map((row) => Math.round(gamma(Math.max(0, Math.min(1, row[0] * v[0] + row[1] * v[1] + row[2] * v[2])))));
};

// Liest berechnete Farben (beliebiges CSS-Farbformat → RGB über ein Canvas) und die Hintergrundfarbe der Vorfahren
const colorsOf = (page, specs) => page.evaluate((specs) => {
  const ctx = document.createElement('canvas').getContext('2d', { willReadFrequently: true });
  const rgba = (css) => { ctx.clearRect(0, 0, 1, 1); ctx.fillStyle = '#000'; ctx.fillStyle = css; ctx.fillRect(0, 0, 1, 1); const d = ctx.getImageData(0, 0, 1, 1).data; return [d[0], d[1], d[2], d[3] / 255]; };
  const bgOf = (el) => { for (let e = el; e; e = e.parentElement) { const c = rgba(getComputedStyle(e).backgroundColor); if (c[3] > 0.9) return c.slice(0, 3); } return rgba(getComputedStyle(document.body).backgroundColor).slice(0, 3); };
  return specs.map(([sel, what]) => {
    const el = document.querySelector(sel);
    if (!el) return null;
    const s = getComputedStyle(el);
    const parentBg = el.parentElement ? bgOf(el.parentElement) : [255, 255, 255];
    const own = rgba(s.backgroundColor);
    return { sel, what, color: rgba(s.color).slice(0, 3), bg: own[3] > 0.9 ? own.slice(0, 3) : bgOf(el), border: rgba(s.borderTopColor).slice(0, 3), parentBg };
  });
}, specs);

const PAGES = ['/', '/2020/10/kurbis/', '/tag/wolle/', '/impressum/'];

for (const scheme of ['light', 'dark']) {
  await test(`Kontraste im ${scheme === 'light' ? 'Hell' : 'Dunkel'}modus (WCAG AA) auf allen Seitentypen`, async () => {
    for (const path of PAGES) {
      const page = await newPage(scheme, path);
      const found = (await colorsOf(page, [
        ['main p', 'text'], ['main a', 'link'], ['.entry-meta', 'muted'], ['.cloud a', 'cloud'], ['.topnav > ul > li > a', 'nav'],
        ['.searchbox button', 'lens'], ['.archive summary', 'archive'], ['#blogfooter .footer-links a', 'footer'], ['#blogfooter .copyright', 'copyright'],
      ])).filter(Boolean);
      for (const f of found) {
        const r = ratio(f.color, f.bg);
        assert.ok(r >= 4.5, `${path} ${f.what}: Kontrast ${r.toFixed(2)} (${f.color} auf ${f.bg})`);
        if (['link', 'text', 'muted'].includes(f.what)) {
          for (const [name, m] of Object.entries(MACHADO)) {
            const rc = ratio(simulate(f.color, m), simulate(f.bg, m));
            assert.ok(rc >= 4.5, `${path} ${f.what} bei ${name}: ${rc.toFixed(2)}`);
          }
        }
        if (f.what === 'lens') for (const [name, m] of Object.entries(MACHADO)) assert.ok(ratio(simulate(f.color, m), simulate(f.bg, m)) >= 4.5, `${path} ${f.what} bei ${name}`);
      }
      // Bedienelemente: Rand des Suchfelds und Akzentfläche gegen den Untergrund mindestens 3 : 1
      const ui = (await colorsOf(page, [['.topsearch input[type=search]', 'Suchfeldrand'], ['.searchbox button', 'Akzentfläche']])).filter(Boolean);
      const field = ui.find((u) => u.what === 'Suchfeldrand');
      assert.ok(ratio(field.border, field.parentBg) >= 3, `${path}: Suchfeldrand ${ratio(field.border, field.parentBg).toFixed(2)}`);
      const lens = ui.find((u) => u.what === 'Akzentfläche');
      assert.ok(ratio(lens.bg, field.bg) >= 3, `${path}: Akzent gegen Karte ${ratio(lens.bg, field.bg).toFixed(2)}`);
      await page.context().close();
    }
  });

  await test(`${scheme}: Seite, Karten und Footer haben passende Hintergründe`, async () => {
    const page = await newPage(scheme, '/');
    const [bodyBg, cardBg, footerBg] = await page.evaluate(() => [getComputedStyle(document.body).backgroundColor, getComputedStyle(document.querySelector('.cloud-card')).backgroundColor, getComputedStyle(document.querySelector('#blogfooter')).backgroundColor]);
    const parse = (c) => c.match(/[\d.]+/g).slice(0, 3).map(Number);
    if (scheme === 'dark') {
      assert.ok(lum(parse(bodyBg)) < 0.05 && lum(parse(cardBg)) < 0.1, `dunkel: ${bodyBg} / ${cardBg}`);
      assert.ok(lum(parse(cardBg)) > lum(parse(bodyBg)), 'Karte heller als Seite');
    } else {
      assert.ok(lum(parse(bodyBg)) > 0.8 && lum(parse(cardBg)) >= lum(parse(bodyBg)) - 0.001, `hell: ${bodyBg} / ${cardBg}`);
    }
    assert.ok(lum(parse(footerBg)) < 0.05, `Footer dunkel in beiden Modi: ${footerBg}`);
    assert.equal(await page.evaluate(() => getComputedStyle(document.documentElement).colorScheme), scheme);
    await page.context().close();
  });
}

await test('Flauschi-Nebel: Violett als Akzent, Seitenfarben wie festgelegt', async () => {
  const hell = await newPage('light', '/');
  assert.equal(await hell.evaluate(() => getComputedStyle(document.body).backgroundColor), 'rgb(246, 243, 252)');
  assert.equal(await hell.evaluate(() => getComputedStyle(document.querySelector('.searchbox button')).backgroundColor), 'rgb(109, 79, 224)');
  await hell.context().close();
  const dunkel = await newPage('dark', '/');
  assert.equal(await dunkel.evaluate(() => getComputedStyle(document.body).backgroundColor), 'rgb(14, 11, 31)');
  assert.equal(await dunkel.evaluate(() => getComputedStyle(document.querySelector('.searchbox button')).backgroundColor), 'rgb(143, 115, 245)');
  await dunkel.context().close();
});

await test('Ohne JavaScript folgt die Seite der Systemeinstellung, der Umschalter fehlt', async () => {
  for (const scheme of ['light', 'dark']) {
    const page = await newPage(scheme, '/', { javaScriptEnabled: false });
    assert.equal(await page.locator('.theme-toggle').isVisible(), false);
    const bg = await page.evaluate(() => getComputedStyle(document.body).backgroundColor);
    assert.equal(bg, scheme === 'dark' ? 'rgb(14, 11, 31)' : 'rgb(246, 243, 252)');
    await page.context().close();
  }
});

await test('Umschalter: Automatisch, Hell, Dunkel, Wahl bleibt nach dem Neuladen', async () => {
  const page = await newPage('light', '/');
  const btn = page.locator('.theme-toggle');
  assert.ok(await btn.isVisible());
  const bg = () => page.evaluate(() => getComputedStyle(document.body).backgroundColor);
  const state = () => btn.getAttribute('data-state');
  assert.equal(await state(), 'auto');
  assert.match(await btn.getAttribute('aria-label'), /Automatisch/);
  const box = await btn.boundingBox();
  assert.ok(box.width >= 44 && box.height >= 44, `Klickfläche ${box.width}×${box.height}`);
  await btn.click(); // Hell
  assert.equal(await state(), 'light');
  assert.match(await btn.getAttribute('aria-label'), /Hell/);
  assert.equal(await bg(), 'rgb(246, 243, 252)');
  await btn.click(); // Dunkel trotz heller Systemeinstellung
  assert.equal(await state(), 'dark');
  assert.equal(await bg(), 'rgb(14, 11, 31)');
  assert.equal(await page.evaluate(() => document.documentElement.dataset.theme), 'dark');
  await page.reload();
  assert.equal(await bg(), 'rgb(14, 11, 31)', 'bleibt nach dem Neuladen dunkel');
  assert.equal(await btn.getAttribute('data-state'), 'dark');
  await page.goto(BASE + '/2020/10/kurbis/');
  assert.equal(await bg(), 'rgb(14, 11, 31)', 'gilt auch auf anderen Seiten');
  await btn.click(); // zurück auf Automatisch
  assert.equal(await state(), 'auto');
  assert.equal(await bg(), 'rgb(246, 243, 252)');
  assert.equal(await page.evaluate(() => localStorage.getItem('theme')), null);
  await page.context().close();
});

await test('Umschalter: Tastatur, Fokus sichtbar, kein Aufblitzen beim Laden', async () => {
  const page = await newPage('light', '/');
  await page.locator('.theme-toggle').focus();
  const outline = await page.evaluate(() => { const s = getComputedStyle(document.activeElement); return [s.outlineStyle, parseFloat(s.outlineWidth)]; });
  assert.ok(outline[0] !== 'none' && outline[1] > 0);
  await page.keyboard.press('Enter');
  await page.keyboard.press('Enter');
  assert.equal(await page.locator('.theme-toggle').getAttribute('data-state'), 'dark');
  // Aufblitzen: frische Seite im gleichen Kontext; schon beim DOMContentLoaded ist das Attribut gesetzt (Skript im head)
  const flash = await page.evaluate(async () => {
    const r = await fetch('/');
    const html = await r.text();
    return html.indexOf('localStorage.getItem("theme")') < html.indexOf('<link rel="stylesheet"');
  });
  assert.ok(flash, 'Skript steht vor dem Stylesheet im head');
  const early = await newPage('light', '/');
  await early.evaluate(() => localStorage.setItem('theme', 'dark'));
  const ctx = early.context();
  const p2 = await ctx.newPage();
  await p2.addInitScript(() => { document.addEventListener('DOMContentLoaded', () => { window.__bgAtDcl = getComputedStyle(document.body).backgroundColor; }); });
  await p2.goto(BASE + '/');
  assert.equal(await p2.evaluate(() => window.__bgAtDcl), 'rgb(14, 11, 31)', 'Hintergrund ist beim Laden schon dunkel');
  await ctx.close();
  await page.context().close();
});

await test('Sterne: dunkel sichtbar, hell zart, hinter dem Inhalt, ohne Mausereignisse; Inhalt auf einfarbigem Grund', async () => {
  for (const [scheme, min, max] of [['dark', 0.99, 1.01], ['light', 0.2, 0.5]]) {
    const page = await newPage(scheme, '/');
    const info = await page.evaluate(() => {
      const s = getComputedStyle(document.body, '::before');
      const c = getComputedStyle(document.querySelector('div.container-fluid'), '::before');
      return { z: s.zIndex, events: s.pointerEvents, opacity: parseFloat(s.opacity), mask: s.maskImage || s.webkitMaskImage, pos: s.position, panelBg: c.backgroundImage, anim: getComputedStyle(document.body, '::before').animationName };
    });
    assert.equal(info.z, '-1');
    assert.equal(info.events, 'none');
    assert.equal(info.pos, 'fixed');
    assert.ok(info.opacity >= min && info.opacity <= max, `${scheme}: Deckkraft ${info.opacity}`);
    assert.match(info.mask, /stars-small\.svg/);
    assert.equal(info.anim, 'none', 'keine Animation');
    // Stern-Dekoration blockiert keinen Klick
    await page.click('.entry-title a >> nth=0');
    await page.waitForURL(/\/20\d\d\/\d\d\//);
    await page.context().close();
  }
});

await test('Erhöhter Kontrast: keine Sterne und kein Nebel', async () => {
  const ctx = await browser.newContext({ colorScheme: 'dark', contrast: 'more', viewport: { width: 1440, height: 900 } });
  const page = await ctx.newPage();
  await page.goto(BASE + '/');
  const [display, image] = await page.evaluate(() => [getComputedStyle(document.body, '::before').display, getComputedStyle(document.body).backgroundImage]);
  assert.equal(display, 'none');
  assert.equal(image, 'none');
  await ctx.close();
});

await test('Suche und Topbar im Dunkelmodus dunkel mit hellem Text', async () => {
  const page = await newPage('dark', '/suche/?q=m%C3%BCtze');
  await page.waitForSelector('.pagefind-ui__result');
  const [text, bg] = await page.evaluate(() => { const t = document.querySelector('.pagefind-ui__result-title a'); const f = document.querySelector('.pagefind-ui'); return [getComputedStyle(t).color, getComputedStyle(f).getPropertyValue('--pagefind-ui-background').trim()]; });
  const rgb = text.match(/\d+/g).slice(0, 3).map(Number);
  assert.ok(lum(rgb) > 0.3, `Treffertitel hell: ${text}`);
  assert.ok(bg !== '' && !/^#fff/i.test(bg), `Suchoberfläche dunkel (${bg})`);
  const barBg = await page.evaluate(() => getComputedStyle(document.querySelector('.topbar')).backgroundColor);
  assert.ok(lum(barBg.match(/\d+/g).slice(0, 3).map(Number)) < 0.1, `Topbar dunkel: ${barBg}`);
  await page.screenshot({ path: '/tmp/flausch-theme-suche-dunkel.png' });
  await page.context().close();
});

await test('Maskottchen im Dunkelmodus mit Schein', async () => {
  const page = await newPage('dark', '/');
  assert.match(await page.evaluate(() => getComputedStyle(document.querySelector('.mascot-normal')).filter), /drop-shadow/);
  await page.context().close();
  const hell = await newPage('light', '/');
  assert.equal(await hell.evaluate(() => getComputedStyle(document.querySelector('.mascot-normal')).filter), 'none');
  await hell.context().close();
});

await test('Handy 360 px: Umschalter neben dem Hamburger-Knopf, kein waagerechtes Scrollen, beide Modi', async () => {
  for (const scheme of ['light', 'dark']) {
    const page = await newPage(scheme, '/', { viewport: { width: 360, height: 740 }, hasTouch: true, isMobile: true });
    const [t, h] = [await page.locator('.theme-toggle').boundingBox(), await page.locator('details.menu-toggle > summary').boundingBox()];
    assert.ok(t.x + t.width <= h.x + 1, 'Umschalter links vom Hamburger');
    assert.ok(Math.abs((t.y + t.height / 2) - (h.y + h.height / 2)) <= 3, 'auf einer Linie');
    const [sw, cw] = await page.evaluate(() => [document.documentElement.scrollWidth, document.documentElement.clientWidth]);
    assert.ok(sw <= cw, `${scheme}: kein waagerechtes Scrollen`);
    await page.screenshot({ path: `/tmp/flausch-theme-handy-${scheme}.png` });
    await page.context().close();
  }
});

await test('Screenshots für die Abnahme', async () => {
  for (const scheme of ['light', 'dark']) {
    for (const [path, name] of [['/', 'start'], ['/2016/05/alltagshute/', 'beitrag'], ['/beitraege/', 'beitraege'], ['/tag/wolle/', 'thema']]) {
      const page = await newPage(scheme, path);
      await page.screenshot({ path: `/tmp/flausch-theme-${scheme}-${name}.png` });
      await page.context().close();
    }
  }
});

await browser.close();
for (const r of results) console.log(r.join(' – '));
process.exit(results.every((r) => r[0] === 'ok') ? 0 : 1);
