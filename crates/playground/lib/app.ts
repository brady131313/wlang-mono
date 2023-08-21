import { WorkoutCst } from "playground";

const cst = new WorkoutCst("# Bency Press");
console.log(cst.toString());

// ESBuild live refresh
new EventSource("/esbuild").addEventListener("change", () => location.reload());
