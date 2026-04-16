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

// Knapper: send tastatur-events til canvas ved trykk/slipp
for (const btn of document.querySelectorAll("[data-key]")) {
  for (const [pointer, keyboard] of [["pointerdown", "keydown"], ["pointerup", "keyup"]]) {
    btn.addEventListener(pointer, (e) => {
      e.preventDefault();
      document.dispatchEvent(new KeyboardEvent(keyboard, { key: btn.dataset.key }));
    });
  }
}

const ejectButton = document.getElementById("eject-button");
ejectButton.addEventListener("click", () => {
  localStorage.removeItem('rom-title');
  localStorage.removeItem('rom-data');
  document.getElementById("game-menu").hidePopover();
  location.reload();
});
