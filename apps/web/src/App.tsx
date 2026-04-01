import { useEffect, useRef, useState } from "react";
import { drawCanvas } from "./canvas";
import { initWasm, renderClock, renderProjectile, renderCircle } from "./wasm";

export default function App() {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const [ready, setReady] = useState(false);

    useEffect(() => {
        initWasm().then(() => setReady(true));
    }, []);

    function renderScene(fn: () => { width: number; height: number; pixels: Uint8ClampedArray }) {
        if (!canvasRef.current) return;

        const result = fn();

        drawCanvas(canvasRef.current, result.width, result.height, result.pixels);
    }

    return (
        <div>
            <h1>Ray Tracer Demo</h1>

            <div style={{ display: "flex", gap: "8px" }}>
                <button disabled={!ready} onClick={() => renderScene(renderProjectile)}>
                    Render Projectile
                </button>

                <button disabled={!ready} onClick={() => renderScene(renderClock)}>
                    Render Clock
                </button>

                <button disabled={!ready} onClick={() => renderScene(renderCircle)}>
                    Render Circle
                </button>
            </div>

            <div style={{ marginTop: "12px" }}>
                <canvas ref={canvasRef} style={{ border: "1px solid #ccc" }} />
            </div>
        </div>
    );
}
