import { defineConfig, searchForWorkspaceRoot } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig(({ command }) => ({
    base: command === "build" ? "/ray-tracer/" : "/",
    plugins: [react()],
    server: {
        fs: {
            allow: [searchForWorkspaceRoot(process.cwd()), "../../crates/wasm/pkg"],
        },
    },
}));
