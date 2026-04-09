# Fremgangsmåte: Porter Game Boy-emulatoren til WASM

Emulatoren kjører native med `pixels` + `winit`. Målet er å få den til å også kjøre i nettleseren via WebAssembly, uten å bryte native-bygget.

Vi bruker et Cargo workspace for å skille emulatorkjernen fra frontendene. Sluttresultatet ser slik ut:

```
gameboy/
├── Cargo.toml              (workspace)
├── core/                   (emulatorkjernen — ingen GUI-avhengigheter)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── cpu/
│       ├── ppu/
│       ├── mbc/
│       ├── game_boy.rs
│       ├── cartridge.rs
│       └── ...
├── native/                 (desktop-frontend)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── web/                    (WASM-frontend)
    ├── Cargo.toml
    ├── src/
    │   └── lib.rs
    ├── index.html
    └── index.js
```

De tre første stegene er generelle forbedringer som gir mening uavhengig av WASM. De kan gjøres på `main`-greina som vanlige refaktoreringer og oppgraderinger. De resterende stegene er WASM-spesifikke.

---

## Del 1: Generelle forbedringer

### 1. Sett opp workspace og trekk ut emulatorkjernen

Modulene (`cpu`, `game_boy`, `cartridge`, osv.) er i dag bare tilgjengelige fra `main.rs` via `mod`-deklarasjoner. Trekk dem ut i en egen crate for å skille emulatorkjernen fra presentasjonslaget.

- Opprett `core/` med egen `Cargo.toml` og `src/lib.rs` med `pub mod`-deklarasjoner for alle emulatormodulene
- Flytt alle emulatormodulene fra `src/` til `core/src/`
- Opprett `native/` med egen `Cargo.toml` og flytt `main.rs` dit
- Opprett workspace `Cargo.toml` på rotnivå med `members = ["core", "native"]`
- I `native/Cargo.toml`: legg til `gameboy-core = { path = "../core" }` som avhengighet
- I `native/src/main.rs`: erstatt `mod`-deklarasjonene med `use gameboy_core::*`
- Konstantene `SCREEN_WIDTH` og `SCREEN_HEIGHT` kan legges i `core`

### 2. Separer I/O fra konstruksjon

I dag leser `Cartridge::from_path` fra filsystemet og oppretter cartridge-objektet i ett steg. Ved å separere disse får du en renere konstruktør som ikke er knyttet til filsystemet.

- **`cartridge.rs`**: Lag en `from_bytes(data: Vec<u8>, save_path: Option<PathBuf>)` og la `from_path` kalle denne.
- **`cpu.rs`**: Lag en `from_cartridge(cartridge: Cartridge)` og la `new` kalle denne.
- **`game_boy.rs`**: Lag en `from_bytes(data: Vec<u8>)` som bruker de nye konstruktørene.

### 3. Oppgrader `pixels` og `winit`

- Oppgrader til `pixels` 0.16 og `winit` 0.30 i `native/Cargo.toml`
- Tilpass koden i `native/src/main.rs` til API-endringene (f.eks. `WindowBuilder` → `Window::default_attributes`, `Arc<Window>`, `event_loop.create_window()`)
- Sjekk at native-bygget fortsatt fungerer

---

## Del 2: WASM-portering

### 4. Opprett `web`-craten

Legg til en ny crate i workspacet for WASM-frontenden.

- Opprett `web/Cargo.toml` med `crate-type = ["cdylib"]` og avhengigheter:
  - `gameboy-core = { path = "../core" }`
  - `pixels`, `winit`
  - `wasm-bindgen`, `wasm-bindgen-futures`
  - `web-sys` (med features: `Document`, `Element`, `Performance`)
  - `console_log`, `console_error_panic_hook`
- Legg til `"web"` i workspace-medlemmene i rot-`Cargo.toml`

### 5. Skriv WASM-entrypointet (`web/src/lib.rs`)

Lag en `#[wasm_bindgen]`-funksjon som tar inn ROM-data og starter emuleringen.

Nøkkelpunkter:
- Sett opp panic hook og logging (`console_error_panic_hook`, `console_log`)
- Bruk `wasm_bindgen_futures::spawn_local` for async-konteksten
- Opprett event loop og vindu med winit (winit håndterer canvas på web)
- Hent canvas fra winit-vinduet med `window.canvas()` og legg den til i DOM
- Bygg `Pixels` med `PixelsBuilder::build_async().await` (ikke `Pixels::new`)
- Du kan trenge å sette `surface_texture_format` eksplisitt (Bgra8Unorm)
- Bruk `web_sys::Performance` for timing i stedet for `std::time::Instant`
- Begrens tidssteg for å unngå spiral ved tab-bytte

### 6. Lag web-frontenden

- `web/index.html`: En side med en `<div id="screen">` (der canvas plasseres), en filinput for ROM, og litt styling
- `web/index.js`: Importer WASM-pakken med `import init, { start_emulator } from './pkg/gameboy_web.js'`, les filen som `Uint8Array`, og kall `start_emulator(bytes)`

### 7. Bygg og test

```bash
# Native
cargo run --release -p gameboy

# WASM
wasm-pack build web/ --target web --out-dir pkg

# Serve
cd web && python -m http.server 8080
```
