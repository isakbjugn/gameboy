import { html, render } from './vendor/htm-preact.js';
import init, { main as startEmulator } from "./pkg/gameboy_web.js";
import { App } from './components/App.js';

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

render(html`<${App} startEmulator=${startEmulator} />`, document.getElementById("app"));
