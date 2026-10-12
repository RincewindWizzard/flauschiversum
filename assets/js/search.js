// Suchseite: startet die Pagefind-Oberfläche (vom eigenen Server), liest ?q= und hält die Adresse aktuell.
// Ohne JavaScript bleibt das einfache Formular, das /suche/?q=… neu lädt, plus Links zum Stöbern.
const params = new URLSearchParams(location.search);
const query = params.get('q') ?? '';
const form = document.getElementById('search-form');
const host = document.getElementById('search');

if (host && typeof PagefindUI === 'function') {
  form.hidden = true;
  host.hidden = false;
  const ui = new PagefindUI({
    element: host,
    showImages: true,
    showSubResults: false,
    showEmptyFilters: false,
    openFilters: ['Thema'],
    resetStyles: false,
    autofocus: !query,
    pageSize: 10,
  });
  const input = host.querySelector('input[type="text"], input[type="search"]');
  if (query) ui.triggerSearch(query);
  // Die Adresse zeigt immer die aktuelle Suche und lässt sich so teilen oder neu laden
  input?.addEventListener('input', () => {
    const url = new URL(location.href);
    if (input.value) url.searchParams.set('q', input.value);
    else url.searchParams.delete('q');
    history.replaceState(null, '', url);
  });
} else if (query) {
  form.querySelector('input[name="q"]').value = query;
}
