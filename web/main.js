import init, { main } from "./pkg/gameboy_web.js";

await init();

// Videresend tastatur-events til canvas slik at winit
// fanger dem opp uansett hvilket element som har fokus.
for (const type of ["keydown", "keyup"]) {
  document.addEventListener(type, (e) => {
    const canvas = document.querySelector("#screen canvas");
    if (canvas && e.target !== canvas) {
      canvas.dispatchEvent(new KeyboardEvent(type, e));
    }
  });
}

const romDropZone = document.getElementById("rom-drop-zone");
const fileInput = document.getElementById("game-pak-input");

const romTitleFromLocalStorage = localStorage.getItem('rom-title');
const romDataFromLocalStorage = localStorage.getItem('rom-data');
if (romTitleFromLocalStorage && romDataFromLocalStorage) {
  const romFile = new Uint8Array(JSON.parse(romDataFromLocalStorage));
  await loadRom(romTitleFromLocalStorage, romFile);
}

window.addEventListener("drop", (e) => {
  if ([...e.dataTransfer.items].some((item) => item.kind === "file")) {
    e.preventDefault();
  }
});

window.addEventListener("dragover", (e) => {
  const fileItems = [...e.dataTransfer.items].filter(
    (item) => item.kind === "file",
  );
  if (fileItems.length > 0) {
    e.preventDefault();
    if (!romDropZone.contains(e.target)) {
      e.dataTransfer.dropEffect = "none";
    }
  }
});

romDropZone.addEventListener("dragover", (e) => {
  e.preventDefault();
  e.dataTransfer.dropEffect = "copy";
});

romDropZone.addEventListener("drop", async (e) => {
  e.preventDefault();
  const rom_file = e.dataTransfer.files[0];
  if (!rom_file) {
    return;
  }

  const game_title = rom_file.name.split(".")[0];
  const bytes = new Uint8Array(await rom_file.arrayBuffer());
  await loadRom(game_title, bytes);
});

fileInput.addEventListener("change", async (e) => {
  const rom_file = e.target.files[0];
  if (!rom_file) {
    return;
  }
  const game_title = rom_file.name.split(".")[0];
  const bytes = new Uint8Array(await rom_file.arrayBuffer());
  await loadRom(game_title, bytes);
});

async function loadRom(romTitle, romData) {
  romDropZone.style.display = "none";
  localStorage.setItem('rom-title', romTitle);
  localStorage.setItem('rom-data', JSON.stringify(Array.from(romData)));
  main(romTitle, romData);
}

// Knapper: multi-touch-støtte for å kunne trykke flere knapper samtidig.
// Sporer hvilke taster som er aktive per touch-punkt, og sender
// keydown/keyup-events når fingre treffer eller forlater knapper.
const activeKeys = new Map(); // touch-identifier → data-key

function keyForTouch(touch) {
  const el = document.elementFromPoint(touch.clientX, touch.clientY);
  return el?.closest("[data-key]")?.dataset.key ?? null;
}

function syncTouches(e) {
  // Bare preventDefault for touch på spillknapper, slik at meny etc. fungerer normalt.
  const touchesGameButton = [...e.touches].some((t) => {
    const el = document.elementFromPoint(t.clientX, t.clientY);
    return el?.closest("[data-key]");
  });
  if (touchesGameButton) e.preventDefault();
  const currentTouches = new Set();
  for (const touch of e.touches) {
    currentTouches.add(touch.identifier);
    const key = keyForTouch(touch);
    const prev = activeKeys.get(touch.identifier);
    if (prev !== key) {
      if (prev) document.dispatchEvent(new KeyboardEvent("keyup", { key: prev }));
      if (key) document.dispatchEvent(new KeyboardEvent("keydown", { key }));
      if (key) activeKeys.set(touch.identifier, key);
      else activeKeys.delete(touch.identifier);
    }
  }
  // Fjern touch-punkter som ikke lenger finnes (touchend/touchcancel)
  for (const [id, key] of activeKeys) {
    if (!currentTouches.has(id)) {
      document.dispatchEvent(new KeyboardEvent("keyup", { key }));
      activeKeys.delete(id);
    }
  }
}

const gameboy = document.querySelector(".gameboy");
for (const event of ["touchstart", "touchmove", "touchend", "touchcancel"]) {
  gameboy.addEventListener(event, syncTouches, { passive: false });
}

// Fallback for desktop (mus/pointer)
if (!("ontouchstart" in window)) {
  for (const btn of document.querySelectorAll("[data-key]")) {
    for (const [pointer, keyboard] of [["pointerdown", "keydown"], ["pointerup", "keyup"]]) {
      btn.addEventListener(pointer, (e) => {
        e.preventDefault();
        document.dispatchEvent(new KeyboardEvent(keyboard, { key: btn.dataset.key }));
      });
    }
  }
}

const resetComboButton = document.getElementById("reset-combo-button");
resetComboButton.addEventListener("click", () => {
  document.getElementById("game-menu").hidePopover();
  const keys = ["Enter", "Backspace", "z", "x"];
  for (const key of keys) {
    document.dispatchEvent(new KeyboardEvent("keydown", { key }));
  }
  setTimeout(() => {
    for (const key of keys) {
      document.dispatchEvent(new KeyboardEvent("keyup", { key }));
    }
  }, 200);
});

const ejectButton = document.getElementById("eject-button");
ejectButton.addEventListener("click", () => {
  localStorage.removeItem('rom-title');
  localStorage.removeItem('rom-data');
  document.getElementById("game-menu").hidePopover();
  location.reload();
});
