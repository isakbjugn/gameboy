import { html, useState, useEffect } from '../vendor/htm-preact.js';
import { Menu } from './Menu.js';
import { Screen } from './Screen.js';
import { RomDropZone } from './RomDropZone.js';

export function App({ startEmulator }) {
  const [romLoaded, setRomLoaded] = useState(false);

  useEffect(() => {
    const savedTitle = localStorage.getItem('rom-title');
    const savedData = localStorage.getItem('rom-data');
    if (savedTitle && savedData) {
      const romData = new Uint8Array(JSON.parse(savedData));
      setRomLoaded(true);
      startEmulator(savedTitle, romData);
    }
  }, []);

  function loadRom(title, data) {
    localStorage.setItem('rom-title', title);
    localStorage.setItem('rom-data', JSON.stringify(Array.from(data)));
    setRomLoaded(true);
    startEmulator(title, data);
  }

  function ejectRom() {
    localStorage.removeItem('rom-title');
    localStorage.removeItem('rom-data');
    location.reload();
  }

  return html`
    <${Menu} onEject=${ejectRom} />
    <${Screen}>
      ${!romLoaded && html`<${RomDropZone} onRomLoaded=${loadRom} />`}
    <//>
  `;
}
