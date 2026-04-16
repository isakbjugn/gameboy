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
const activeKeys = new Map(); // touch-identifier → Set<key>

// Geometrisk d-pad: regn ut aktive retninger basert på vinkel fra midtpunktet.
// Deler sirkelen i 8 sektorer à 45°. Kardinalsektorer gir én retning,
// diagonalsektorer gir to naboretninger samtidig.
function dpadKeysForTouch(touch, dpadEl) {
  const rect = dpadEl.getBoundingClientRect();
  const cx = rect.left + rect.width / 2;
  const cy = rect.top + rect.height / 2;
  const dx = touch.clientX - cx;
  const dy = touch.clientY - cy;

  // Vinkel i grader, 0° = høyre, positiv med klokka
  let angle = Math.atan2(dy, dx) * (180 / Math.PI);
  if (angle < 0) angle += 360;

  // 8 sektorer à 45°, med sektor 0 sentrert rundt 0° (høyre)
  //   0: høyre        (337.5–22.5)
  //   1: ned+høyre    (22.5–67.5)
  //   2: ned          (67.5–112.5)
  //   3: ned+venstre  (112.5–157.5)
  //   4: venstre      (157.5–202.5)
  //   5: opp+venstre  (202.5–247.5)
  //   6: opp          (247.5–292.5)
  //   7: opp+høyre    (292.5–337.5)
  const sector = Math.floor(((angle + 22.5) % 360) / 45);
  const map = [
    ["ArrowRight"],                  // 0
    ["ArrowDown", "ArrowRight"],     // 1
    ["ArrowDown"],                   // 2
    ["ArrowDown", "ArrowLeft"],      // 3
    ["ArrowLeft"],                   // 4
    ["ArrowUp", "ArrowLeft"],        // 5
    ["ArrowUp"],                     // 6
    ["ArrowUp", "ArrowRight"],       // 7
  ];
  return map[sector];
}

function keysForTouch(touch) {
  const el = document.elementFromPoint(touch.clientX, touch.clientY);
  const dpad = el?.closest("[data-dpad]");
  if (dpad) return dpadKeysForTouch(touch, dpad);
  const key = el?.closest("[data-key]")?.dataset.key;
  return key ? [key] : [];
}

// Hjelpefunksjon: sammenlign to sorterte nøkkelsett
function sameKeys(a, b) {
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) if (a[i] !== b[i]) return false;
  return true;
}

function syncTouches(e) {
  // Bare preventDefault for touch på spillknapper, slik at meny etc. fungerer normalt.
  const touchesGameButton = [...e.touches].some((t) => {
    const el = document.elementFromPoint(t.clientX, t.clientY);
    return el?.closest("[data-key]") || el?.closest("[data-dpad]");
  });
  if (touchesGameButton) e.preventDefault();
  const currentTouches = new Set();
  for (const touch of e.touches) {
    currentTouches.add(touch.identifier);
    const keys = keysForTouch(touch);
    const prev = activeKeys.get(touch.identifier) ?? [];
    if (!sameKeys(prev, keys)) {
      // Slipp taster som ikke lenger er aktive
      for (const k of prev) {
        if (!keys.includes(k)) document.dispatchEvent(new KeyboardEvent("keyup", { key: k }));
      }
      // Trykk ned nye taster
      for (const k of keys) {
        if (!prev.includes(k)) {
          document.dispatchEvent(new KeyboardEvent("keydown", { key: k }));
          navigator.vibrate?.(15);
        }
      }
      if (keys.length) activeKeys.set(touch.identifier, keys);
      else activeKeys.delete(touch.identifier);
    }
  }
  // Fjern touch-punkter som ikke lenger finnes (touchend/touchcancel)
  for (const [id, keys] of activeKeys) {
    if (!currentTouches.has(id)) {
      for (const k of keys) document.dispatchEvent(new KeyboardEvent("keyup", { key: k }));
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
  // D-pad med mus: geometrisk retningsberegning
  const dpadOverlay = document.querySelector("[data-dpad]");
  if (dpadOverlay) {
    let activeDpadKeys = [];
    dpadOverlay.addEventListener("pointerdown", (e) => {
      e.preventDefault();
      dpadOverlay.setPointerCapture(e.pointerId);
      activeDpadKeys = dpadKeysForTouch(e, dpadOverlay);
      for (const k of activeDpadKeys) document.dispatchEvent(new KeyboardEvent("keydown", { key: k }));
    });
    dpadOverlay.addEventListener("pointermove", (e) => {
      if (!dpadOverlay.hasPointerCapture(e.pointerId)) return;
      const keys = dpadKeysForTouch(e, dpadOverlay);
      if (!sameKeys(activeDpadKeys, keys)) {
        for (const k of activeDpadKeys) {
          if (!keys.includes(k)) document.dispatchEvent(new KeyboardEvent("keyup", { key: k }));
        }
        for (const k of keys) {
          if (!activeDpadKeys.includes(k)) document.dispatchEvent(new KeyboardEvent("keydown", { key: k }));
        }
        activeDpadKeys = keys;
      }
    });
    dpadOverlay.addEventListener("pointerup", (e) => {
      for (const k of activeDpadKeys) document.dispatchEvent(new KeyboardEvent("keyup", { key: k }));
      activeDpadKeys = [];
    });
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
