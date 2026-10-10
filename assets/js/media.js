// Karussell, Vollbildansicht (PhotoSwipe) und Videos in Posts.
// Ohne JavaScript bleibt alles nutzbar: Karussells sind scrollbar, Bilder verlinken auf 1000x.
import PhotoSwipeLightbox from '/assets/vendor/photoswipe/photoswipe-lightbox.esm.min.js';

const css = document.createElement('link');
css.rel = 'stylesheet';
css.href = '/assets/vendor/photoswipe/photoswipe.css';
document.head.append(css);

// ---------- Karussell ----------
function setupCarousel(carousel) {
  const track = carousel.querySelector('.carousel-track');
  const slides = [...track.children];
  if (slides.length < 2) return;

  const button = (cls, label, text) => {
    const b = document.createElement('button');
    b.type = 'button';
    b.className = `carousel-arrow ${cls}`;
    b.setAttribute('aria-label', label);
    b.textContent = text;
    carousel.append(b);
    return b;
  };
  const prev = button('prev', 'vorheriges Bild', '‹');
  const next = button('next', 'nächstes Bild', '›');

  const dots = document.createElement('div');
  dots.className = 'carousel-dots';
  const dotButtons = slides.map((_, i) => {
    const d = document.createElement('button');
    d.type = 'button';
    d.setAttribute('aria-label', `Bild ${i + 1} von ${slides.length}`);
    d.addEventListener('click', () => go(i));
    dots.append(d);
    return d;
  });
  carousel.append(dots);

  const current = () => Math.round(track.scrollLeft / track.clientWidth);
  const go = (i) => {
    const target = Math.max(0, Math.min(slides.length - 1, i));
    track.scrollTo({ left: target * track.clientWidth });
  };
  const update = () => {
    const i = current();
    dotButtons.forEach((d, n) => d.setAttribute('aria-current', n === i ? 'true' : 'false'));
    prev.disabled = i === 0;
    next.disabled = i === slides.length - 1;
  };

  prev.addEventListener('click', () => go(current() - 1));
  next.addEventListener('click', () => go(current() + 1));
  track.addEventListener('scroll', () => requestAnimationFrame(update), { passive: true });
  track.tabIndex = 0;
  track.setAttribute('aria-roledescription', 'Karussell');
  track.addEventListener('keydown', (e) => {
    if (e.key === 'ArrowRight') { go(current() + 1); e.preventDefault(); }
    if (e.key === 'ArrowLeft') { go(current() - 1); e.preventDefault(); }
  });
  window.addEventListener('resize', update);
  update();
}

document.querySelectorAll('.carousel').forEach(setupCarousel);

// ---------- Vollbild ----------
// Jede Mediengruppe (Karussell oder Einzelbild) ist eine eigene Galerie.
const galleries = [...document.querySelectorAll('main .carousel, main > figure.media')]
  .filter((g) => g.querySelector('a[data-pswp-width]'));

galleries.forEach((gallery) => {
  const lightbox = new PhotoSwipeLightbox({
    gallery,
    children: 'a[data-pswp-width]',
    pswpModule: () => import('/assets/vendor/photoswipe/photoswipe.esm.min.js'),
    bgOpacity: 0.92,
    closeTitle: 'Schließen (Esc)',
    zoomTitle: 'Zoomen',
    arrowPrevTitle: 'Vorheriges Bild',
    arrowNextTitle: 'Nächstes Bild',
    errorMsg: 'Das Bild konnte nicht geladen werden.',
  });

  // Bildunterschrift aus <figcaption>
  lightbox.on('uiRegister', () => {
    lightbox.pswp.ui.registerElement({
      name: 'caption',
      order: 9,
      isButton: false,
      appendTo: 'root',
      onInit: (el, pswp) => {
        el.className = 'pswp__custom-caption';
        pswp.on('change', () => {
          const fig = pswp.currSlide.data.element?.closest('figure');
          el.textContent = fig?.querySelector('figcaption')?.textContent ?? '';
          el.hidden = !el.textContent;
        });
      },
    });
  });

  // Zurück-Taste des Browsers schließt das Vollbild
  let closedByHistory = false;
  lightbox.on('afterInit', () => {
    closedByHistory = false;
    history.pushState({ pswp: true }, '');
  });
  lightbox.on('close', () => {
    if (!closedByHistory && history.state?.pswp) history.back();
  });
  window.addEventListener('popstate', () => {
    if (lightbox.pswp) {
      closedByHistory = true;
      lightbox.pswp.close();
    }
  });

  lightbox.init();
});

const style = document.createElement('style');
style.textContent = `.pswp__custom-caption{position:absolute;left:50%;bottom:16px;transform:translateX(-50%);
max-width:calc(100% - 32px);padding:4px 10px;border-radius:4px;background:rgba(0,0,0,.55);color:#fff;
font-size:16px;text-align:center}`;
document.head.append(style);

// ---------- Videos ----------
// Bei „Bewegung reduzieren“ starten Videos nicht von selbst, sondern per Tippen.
if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
  document.querySelectorAll('video[autoplay]').forEach((v) => {
    v.pause();
    v.removeAttribute('autoplay');
    v.controls = true;
  });
}
