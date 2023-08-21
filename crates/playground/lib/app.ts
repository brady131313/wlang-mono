import { WorkoutCst } from "playground";
import Playground from "./Playground.svelte";

const playground = new Playground({
  target: document.body,
});

// ESBuild live refresh
new EventSource("/esbuild").addEventListener("change", () => location.reload());
