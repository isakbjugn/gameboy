import init, { start_emulator } from './pkg/gameboy.js';

async function main() {
    await init();

    document.getElementById('rom').addEventListener('change', async (e) => {
        const file = e.target.files[0];
        if (!file) return;
        const buffer = await file.arrayBuffer();
        const bytes = new Uint8Array(buffer);
        start_emulator(bytes);
    });
}

main();
