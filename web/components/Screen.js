import { html } from '../vendor/htm-preact.js';

export function Screen({ children }) {
  return html`
    <div class="bezel">
      <div id="screen"></div>
      ${children}
    </div>
  `;
}
