import { html, useEffect, useRef } from '../vendor/htm-preact.js';

export function RomDropZone({ onRomLoaded }) {
  const zoneRef = useRef(null);

  useEffect(() => {
    const zone = zoneRef.current;

    function preventDefault(e) {
      e.preventDefault();
    }

    function handleWindowDragover(e) {
      const fileItems = [...e.dataTransfer.items].filter(i => i.kind === "file");
      if (fileItems.length > 0) {
        e.preventDefault();
        if (!zone.contains(e.target)) {
          e.dataTransfer.dropEffect = "none";
        }
      }
    }

    function handleWindowDrop(e) {
      if ([...e.dataTransfer.items].some(i => i.kind === "file")) {
        e.preventDefault();
      }
    }

    window.addEventListener("dragover", handleWindowDragover);
    window.addEventListener("drop", handleWindowDrop);

    zone.addEventListener("dragover", (e) => {
      e.preventDefault();
      e.dataTransfer.dropEffect = "copy";
    });

    return () => {
      window.removeEventListener("dragover", handleWindowDragover);
      window.removeEventListener("drop", handleWindowDrop);
    };
  }, []);

  async function handleFile(file) {
    if (!file) return;
    const title = file.name.split(".")[0];
    const bytes = new Uint8Array(await file.arrayBuffer());
    onRomLoaded(title, bytes);
  }

  function handleDrop(e) {
    e.preventDefault();
    handleFile(e.dataTransfer.files[0]);
  }

  function handleChange(e) {
    handleFile(e.target.files[0]);
  }

  return html`
    <label class="rom-drop-zone" ref=${zoneRef} onDrop=${handleDrop}>
      Last opp Game Boy-spillet ditt her
      <input type="file" accept=".gb" onChange=${handleChange} />
    </label>
  `;
}
