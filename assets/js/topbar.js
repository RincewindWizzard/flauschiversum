// Topbar: Farbschema-Umschalter, Handy-Menü (Hamburger) und Archiv „Alle aufklappen“.
// Ohne dieses Skript bleibt alles nutzbar (Menü über `details`, Farbschema nach Systemeinstellung).
(() => {
  const root = document.documentElement;
  root.classList.add("js");

  // Farbschema: Automatisch (Systemeinstellung), Hell, Dunkel; die Wahl wird im Browser gemerkt
  const themeButton = document.querySelector(".theme-toggle");
  if (themeButton) {
    const modes = ["auto", "light", "dark"];
    const names = { auto: "Automatisch", light: "Hell", dark: "Dunkel" };
    const read = () => { try { return localStorage.getItem("theme"); } catch { return null; } };
    const write = (mode) => {
      try { if (mode === "auto") localStorage.removeItem("theme"); else localStorage.setItem("theme", mode); } catch { /* Speicher nicht verfügbar */ }
    };
    const show = (mode) => {
      if (mode === "auto") delete root.dataset.theme; else root.dataset.theme = mode;
      themeButton.dataset.state = mode;
      const label = `Farbschema: ${names[mode]}`;
      themeButton.title = label;
      themeButton.setAttribute("aria-label", `${label}. Zum Wechseln drücken.`);
    };
    const saved = read();
    let mode = modes.includes(saved) ? saved : "auto";
    show(mode);
    themeButton.addEventListener("click", () => {
      mode = modes[(modes.indexOf(mode) + 1) % modes.length];
      write(mode);
      show(mode);
    });
  }

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
