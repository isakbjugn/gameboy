import init, { main } from "./pkg/gameboy_web.js";

const romDropZone = document.getElementById("rom-drop-zone");
const fileInput = document.getElementById("game-pak-input");

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

  await loadRom(rom_file);
});

fileInput.addEventListener("change", async (e) => {
  const rom_file = e.target.files[0];
  if (!rom_file) {
    return;
  }
  await loadRom(rom_file);
});

async function loadRom(file) {
  const game_title = file.name.split(".")[0];
  const bytes = new Uint8Array(await file.arrayBuffer());
  romDropZone.style.display = "none";
  localStorage.setItem('rom', JSON.stringify(Array.from(bytes)));
  main(game_title, bytes);
}

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
