import * as esbuild from "esbuild";
import { wasmLoader } from "esbuild-plugin-wasm";
import sveltePlugin from "esbuild-svelte";
import sveltePreprocess from "svelte-preprocess";

const context = await esbuild.context({
  entryPoints: ["lib/app.ts"],
  bundle: true,
  outdir: "www",
  format: "esm",
  plugins: [wasmLoader(), sveltePlugin({ preprocess: sveltePreprocess() })],
});

await context.watch();
const { host, port } = await context.serve({ servedir: "www" });
console.log(`Listening at ${host}:${port}`);
