import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import { ViteRsw } from "vite-plugin-rsw";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [ViteRsw(), react()],
});
