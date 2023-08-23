import * as esbuild from "esbuild";
import { wasmLoader } from "esbuild-plugin-wasm";

const context = await esbuild.context({
  entryPoints: ["lib/app.ts"],
  bundle: true,
  outdir: "www",
  format: "esm",
  plugins: [wasmLoader()],
});

await context.watch();
const { host, port } = await context.serve({ servedir: "www" });
console.log(`Listening at ${host}:${port}`);
