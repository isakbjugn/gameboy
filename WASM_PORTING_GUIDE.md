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

### 2. Gjør core plattformuavhengig

I dag er `core` knyttet til filsystemet på to måter: `Cartridge::from_path` leser ROM fra disk, og MBC1/MBC3 bruker `std::fs::File` og `PathBuf` direkte for å laste og lagre batteridata (save games). Begge deler må abstraheres bort for at `core` skal kunne brukes på web.

#### 2a. Separer I/O fra konstruksjon

- **`cartridge.rs`**: Lag en `from_bytes`-konstruktør som tar rå ROM-data. La `from_path` kalle denne.
- **`cpu.rs`**: Lag en `from_cartridge(cartridge: Cartridge)` og la `new` kalle denne.
- **`game_boy.rs`**: Lag en `from_bytes` som bruker de nye konstruktørene.

#### 2b. Abstraher batterilagring bak et trait

MBC1 og MBC3 har `battery_save_path: Option<PathBuf>` og bruker `File::open`/`File::create` i konstruktøren og `Drop`. Dette gjør dem avhengige av et ekte filsystem. Løsningen er et trait i `core`:

```rust
// mbc.rs (eller egen fil)
pub trait BatterySave: Send {
    fn load(&self) -> Option<Vec<u8>>;
    fn save(&self, data: &[u8]);
}
```

Endringer:
- **`mbc_1.rs` / `mbc_3.rs`**: Erstatt `has_battery: bool` og `battery_save_path: Option<PathBuf>` med ett felt: `battery: Option<Box<dyn BatterySave>>`. Konstruktøren kaller `battery.load()` i stedet for `File::open`, og `Drop` kaller `battery.save(&self.ram)` i stedet for `File::create`. Fjern `use std::fs::File` og `use std::path::PathBuf`.
- **`cartridge.rs`**: `from_bytes` tar `Option<Box<dyn BatterySave>>` i stedet for `Option<PathBuf>`, og sender den videre til MBC-konstruktørene. `from_path` oppretter en `FileBatterySave` (se under) og kaller `from_bytes`.

Implementasjonene av traitet lever utenfor `core`:
- **Native** (`native/`): `FileBatterySave(PathBuf)` som wrapper `std::fs::read`/`std::fs::write`.
- **Web** (`web/`): `LocalStorageBatterySave { key: String }` som base64-encoder batteridata og lagrer i `localStorage` via `web_sys`.

Etter dette steget skal `core` ikke ha noen `use std::fs` eller `use std::path` — hele craten er plattformuavhengig.

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
