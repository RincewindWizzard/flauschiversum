// Topbar: Untermenüs per Knopf, Hover, Tastatur; Handy-Menü; Archiv „Alle aufklappen“.
// Ohne dieses Skript öffnen die Untermenüs per CSS bei Hover und Fokus.
(() => {
  const root = document.documentElement;
  root.classList.add("js");

  const menus = [...document.querySelectorAll(".topnav li.has-menu")];
  const toggleOf = (li) => li.querySelector(":scope > .submenu-toggle");
  const linksOf = (li) => [...li.querySelectorAll(":scope > .submenu a")];
  const fine = window.matchMedia("(hover: hover)");

  function setOpen(li, open) {
    li.classList.toggle("open", open);
    toggleOf(li).setAttribute("aria-expanded", String(open));
  }
  function closeAll(except) {
    for (const li of menus) if (li !== except) setOpen(li, false);
  }

  for (const li of menus) {
    const button = toggleOf(li);
    let timer;
    let byHover = false; // vom Mauszeiger geöffnet: der erste Klick hält es offen, statt es zu schließen
    button.addEventListener("click", () => {
      const open = byHover ? true : !li.classList.contains("open");
      byHover = false;
      closeAll(li);
      setOpen(li, open);
    });
    li.addEventListener("pointerenter", (e) => {
      if (e.pointerType !== "mouse" || !fine.matches) return;
      clearTimeout(timer);
      closeAll(li);
      byHover = !li.classList.contains("open");
      setOpen(li, true);
    });
    li.addEventListener("pointerleave", (e) => {
      if (e.pointerType !== "mouse") return;
      timer = setTimeout(() => { byHover = false; setOpen(li, false); }, 150);
    });
    li.addEventListener("focusout", (e) => {
      if (!li.contains(e.relatedTarget)) setOpen(li, false);
    });
    li.addEventListener("keydown", (e) => {
      const links = linksOf(li);
      const items = [button, ...links];
      const at = items.indexOf(document.activeElement);
      if (e.key === "Escape" && li.classList.contains("open")) {
        setOpen(li, false);
        button.focus();
        e.preventDefault();
      } else if (e.key === "ArrowDown" && at >= 0) {
        e.preventDefault();
        if (!li.classList.contains("open")) { closeAll(li); setOpen(li, true); }
        items[Math.min(at + 1, items.length - 1)].focus();
      } else if (e.key === "ArrowUp" && at > 0) {
        e.preventDefault();
        items[at - 1].focus();
      }
    });
  }

  // Klick außerhalb und Escape schließen
  document.addEventListener("click", (e) => {
    if (!e.target.closest(".topnav li.has-menu")) closeAll();
  });
  document.addEventListener("keydown", (e) => {
    if (e.key === "Escape") closeAll();
  });

  // Handy-Menü: am Desktop immer offen, am Handy zuerst zu
  const toggle = document.querySelector("details.menu-toggle");
  if (toggle) {
    const mobile = window.matchMedia("(max-width: 47.99em)");
    const sync = () => { toggle.open = !mobile.matches; };
    sync();
    mobile.addEventListener("change", sync);
  }

  // Archiv: Knopf „Alle aufklappen / zuklappen“
  for (const archive of document.querySelectorAll(".archive")) {
    const tools = archive.querySelector(".archive-tools");
    const groups = [...archive.querySelectorAll("details")];
    if (!tools || groups.length === 0) continue;
    const button = document.createElement("button");
    button.type = "button";
    const update = () => {
      button.textContent = groups.every((d) => d.open) ? "Alle zuklappen" : "Alle aufklappen";
    };
    button.addEventListener("click", () => {
      const open = !groups.every((d) => d.open);
      for (const d of groups) d.open = open;
      update();
    });
    for (const d of groups) d.addEventListener("toggle", update);
    update();
    tools.append(button);
    tools.hidden = false;
  }
})();
