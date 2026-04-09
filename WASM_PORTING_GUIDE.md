# Fremgangsmåte: Porter Game Boy-emulatoren til WASM

Emulatoren kjører native med `pixels` + `winit`. Målet er å få den til å også kjøre i nettleseren via WebAssembly, uten å bryte native-bygget.

De tre første stegene er generelle forbedringer som gir mening uavhengig av WASM. De kan gjøres på `main`-greina som vanlige refaktoreringer og oppgraderinger. De resterende stegene er WASM-spesifikke.

---

## Del 1: Generelle forbedringer

### 1. Gjør emulatorkjernen til et bibliotek

Modulene (`cpu`, `game_boy`, `cartridge`, osv.) er i dag bare tilgjengelige fra `main.rs` via `mod`-deklarasjoner. Ved å trekke dem ut i et bibliotek skiller du emulatorkjernen fra presentasjonslaget (vindushåndtering, tastetrykk, osv.).

- Opprett `src/lib.rs` med `pub mod`-deklarasjoner for alle modulene
- Fjern `mod`-deklarasjonene fra `main.rs` og bruk `use gameboy::*` i stedet
- Legg til `[lib]` med `crate-type = ["cdylib", "rlib"]` i `Cargo.toml`

### 2. Separer I/O fra konstruksjon

I dag leser `Cartridge::from_path` fra filsystemet og oppretter cartridge-objektet i ett steg. Ved å separere disse får du en renere konstruktør som ikke er knyttet til filsystemet.

- **`cartridge.rs`**: Lag en `from_bytes(data: Vec<u8>, save_path: Option<PathBuf>)` og la `from_path` kalle denne.
- **`cpu.rs`**: Lag en `from_cartridge(cartridge: Cartridge)` og la `new` kalle denne.
- **`game_boy.rs`**: Lag en `from_bytes(data: Vec<u8>)` som bruker de nye konstruktørene.

### 3. Oppgrader `pixels` og `winit`

- Oppgrader til `pixels` 0.16 og `winit` 0.30
- Tilpass native-koden i `main.rs` til API-endringene (f.eks. `WindowBuilder` → `Window::default_attributes`, `Arc<Window>`, `event_loop.create_window()`)
- Sjekk at native-bygget fortsatt fungerer

---

## Del 2: WASM-portering

### 4. Sett opp betingede avhengigheter i `Cargo.toml`

Noen avhengigheter (som `clap`, `simplelog`) gir ikke mening i WASM, og WASM trenger egne avhengigheter.

- Flytt `clap` og `simplelog` under `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]`
- Legg til WASM-avhengigheter under `[target.'cfg(target_arch = "wasm32")'.dependencies]`:
  - `wasm-bindgen`, `wasm-bindgen-futures`
  - `web-sys` (med features: `Document`, `Element`, `Performance`)
  - `console_log`, `console_error_panic_hook`

### 5. Skriv WASM-entrypointet (`src/wasm.rs`)

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
- Inkluder modulen fra `lib.rs` med `#[cfg(target_arch = "wasm32")]`

### 6. Lag web-frontenden (`web/`)

- `web/index.html`: En side med en `<div id="screen">` (der canvas plasseres), en filinput for ROM, og litt styling
- `web/index.js`: Importer WASM-pakken med `import init, { start_emulator } from './pkg/gameboy.js'`, les filen som `Uint8Array`, og kall `start_emulator(bytes)`

### 7. Bygg og test

```bash
# Native
cargo run --release

# WASM
wasm-pack build --target web --out-dir web/pkg

# Serve
cd web && python -m http.server 8080
```
