import { html, useRef } from '../vendor/htm-preact.js';

export function Menu({ onEject }) {
  const menuRef = useRef(null);

  function handleEject() {
    menuRef.current?.hidePopover();
    onEject();
  }

  return html`
    <button class="menu-button" popovertarget="game-menu" popovertargetaction="toggle" aria-label="Meny">
      \u2630
    </button>
    <div popover id="game-menu" class="game-menu" ref=${menuRef}>
      <button class="menu-item" onClick=${handleEject}>Løs ut kassett</button>
    </div>
  `;
}
