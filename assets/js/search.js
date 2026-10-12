// Suchseite: Suche mit der modularen Pagefind-Oberfläche (vom eigenen Server). Eingabe, Treffer und Themenfilter
// stehen in getrennten Bereichen der Seite (Filter als Seitenleiste). Liest ?q= und hält die Adresse aktuell.
// Ohne JavaScript bleibt der Hinweis mit Links zum Stöbern; das Suchfeld der Topbar schickt ein GET nach /suche/?q=…
const FILTER = 'Thema';
const form = document.getElementById('search-form');
const input = document.getElementById('q');
const results = document.getElementById('search-results');
const summary = document.getElementById('search-summary');
const aside = document.getElementById('search-filters');
const filterList = document.getElementById('search-filter-list');
const filterDetails = document.getElementById('search-filter-details');
const query = new URLSearchParams(location.search).get('q') ?? '';

const el = (tag, attrs = {}, ...children) => {
  const node = document.createElement(tag);
  for (const [k, v] of Object.entries(attrs)) node.setAttribute(k, v);
  node.append(...children);
  return node;
};

if (typeof PagefindModularUI === 'object' && form && results) {
  form.hidden = false;
  const ui = new PagefindModularUI.Instance({ bundlePath: '/pagefind/' });
  const selected = new Set();
  // Auf schmalen Fenstern steht die Filterkarte über den Treffern und ist zunächst zugeklappt
  filterDetails.open = matchMedia('(min-width: 64em)').matches;

  // Treffer: derselbe Aufbau wie die Einträge der Beitragslisten
  const placeholder = () =>
    el('li', { class: 'entry entry-loading', 'aria-hidden': 'true' }, el('div', { class: 'entry-text' }, el('p', { class: 'entry-title' }, ' '), el('p', { class: 'excerpt' }, ' ')));
  const resultItem = (r) => {
    const url = r.meta?.url || r.url;
    const li = el('li', { class: 'entry' });
    if (r.meta?.image) {
      li.append(el('a', { href: url, class: 'entry-image', tabindex: '-1', 'aria-hidden': 'true' }, el('img', { src: r.meta.image, alt: '', loading: 'lazy' })));
    }
    const text = el('div', { class: 'entry-text' }, el('p', { class: 'entry-title' }, el('a', { href: url }, r.meta?.title ?? url)));
    const excerpt = el('p', { class: 'excerpt' });
    excerpt.innerHTML = r.excerpt; // von Pagefind erzeugt: nur Text und <mark>
    text.append(excerpt);
    li.append(text);
    return li;
  };
  ui.add(new PagefindModularUI.ResultList({ containerElement: '#search-results', placeholderTemplate: placeholder, resultTemplate: resultItem }));

  // Meldung zur Trefferzahl
  let term = '';
  ui.on('search', (t) => { term = t; });
  ui.on('loading', () => { if (term) summary.textContent = 'Suche …'; });
  ui.on('results', (r) => {
    summary.replaceChildren();
    if (!term) return;
    const n = r?.results?.length ?? 0;
    if (n) summary.textContent = `${n} Treffer für „${term}“`;
    else summary.append(`Keine Treffer für „${term}“. Stöbere in den `, el('a', { href: '/themen/' }, 'Themen'), '.');
    aside.hidden = !(r?.unfilteredResultCount > 0 || selected.size > 0);
  });

  // Themenfilter in der Seitenleiste: Liste mit Häkchen und Anzahl (bei mehreren Themen müssen alle zutreffen)
  ui.on('filters', ({ total }) => {
    const values = Object.entries(total?.[FILTER] ?? {}).filter(([name, count]) => count > 0 || selected.has(name)).sort((a, b) => a[0].localeCompare(b[0], 'de'));
    filterList.replaceChildren(
      ...values.map(([name, count], i) => {
        const id = `thema-${i}`;
        const box = el('input', { type: 'checkbox', id, value: name });
        box.checked = selected.has(name);
        box.addEventListener('change', () => {
          box.checked ? selected.add(name) : selected.delete(name);
          ui.triggerFilter(FILTER, [...selected]);
        });
        return el('li', {}, box, el('label', { for: id }, `${name} `, el('span', { class: 'count' }, `(${count})`)));
      }),
    );
  });

  // Eingabe: Suche mit kurzer Verzögerung, Adresse aktuell halten (teilbar, neu ladbar)
  let timer;
  const run = () => {
    const value = input.value.trim();
    const url = new URL(location.href);
    value ? url.searchParams.set('q', input.value) : url.searchParams.delete('q');
    history.replaceState(null, '', url);
    ui.triggerSearch(value);
  };
  input.addEventListener('input', () => { clearTimeout(timer); timer = setTimeout(run, 250); });
  input.addEventListener('focus', () => ui.triggerLoad());
  form.addEventListener('submit', (e) => { e.preventDefault(); clearTimeout(timer); run(); });
  input.value = query;
  if (query) ui.triggerSearch(query.trim());
  else input.focus();
}
