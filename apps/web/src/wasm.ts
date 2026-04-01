import init, { render_circle, render_clock, render_projectile } from "ray-tracer-wasm";

let initialized = false;

export async function initWasm() {
    if (initialized) return;
    await init();
    initialized = true;
}

type WasmRenderResult = {
    width: number;
    height: number;
    pixels: Uint8Array;
};

function mapResult(result: WasmRenderResult) {
    return {
        width: result.width,
        height: result.height,
        pixels: new Uint8ClampedArray(result.pixels),
    };
}

export function renderProjectile() {
    return mapResult(render_projectile());
}

export function renderClock() {
    return mapResult(render_clock());
}

export function renderCircle() {
    return mapResult(render_circle());
}
