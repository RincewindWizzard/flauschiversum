// Inhaltsverzeichnis von Einzelseiten: auf schmalen Fenstern zunächst zugeklappt, hebt den Abschnitt hervor, der gerade im Bild ist.
// Ohne JavaScript bleibt das Verzeichnis eine offene Linkliste.
(() => {
  const details = document.getElementById('toc-details');
  const toc = document.getElementById('toc');
  if (!details || !toc) return;
  details.open = matchMedia('(min-width: 64em)').matches;

  const links = [...toc.querySelectorAll('a[href^="#"]')];
  const sections = links
    .map((a) => ({ link: a, heading: document.getElementById(decodeURIComponent(a.hash.slice(1))) }))
    .filter((s) => s.heading);
  if (!sections.length || !('IntersectionObserver' in window)) return;

  // Aktuell ist die letzte Überschrift, die den oberen Rand (unterhalb der Topbar) bereits erreicht hat
  const offset = 100;
  let pinned = null; // Nach Klick auf einen Eintrag gilt dieser, bis wieder gescrollt wird (am Seitenende reicht der Platz sonst nicht)
  const update = () => {
    let current = pinned ?? sections[0];
    if (!pinned) for (const s of sections) if (s.heading.getBoundingClientRect().top - offset <= 0) current = s;
    for (const s of sections) {
      if (s === current) s.link.setAttribute('aria-current', 'true');
      else s.link.removeAttribute('aria-current');
    }
  };
  const observer = new IntersectionObserver(update, { rootMargin: `-${offset}px 0px 0px 0px`, threshold: [0, 1] });
  sections.forEach((s) => observer.observe(s.heading));
  // Der Sprung selbst scrollt; erst wenn die Seite danach um mehr als ein paar Pixel weiterscrollt, gilt wieder die Lesestelle
  let pinY = 0;
  let settle;
  addEventListener('scroll', () => {
    if (pinned) {
      clearTimeout(settle);
      settle = setTimeout(() => { pinY = scrollY; }, 200);
      if (settle && Math.abs(scrollY - pinY) > 60 && pinY !== 0) pinned = null;
    }
    update();
  }, { passive: true });
  for (const s of sections) {
    s.link.addEventListener('click', () => {
      pinned = s;
      pinY = 0;
      clearTimeout(settle);
      settle = setTimeout(() => { pinY = scrollY; }, 200);
      update();
    });
  }
  update();
})();
