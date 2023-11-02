import { logger } from "./lib/utils.ts";
import "./styles.css";
import App from "./App.svelte";

logger.attachConsole();  // Output Rust logs to the web console

const app = new App({
    target: document.getElementById("app"),
});

export default app;
