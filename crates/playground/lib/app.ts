import { createRoot } from "react-dom/client";
import Playground from "./Playground";
import React from "react";

// ESBuild live refresh
new EventSource("/esbuild").addEventListener("change", () => location.reload());

const rootEl = document.querySelector("#root");
const root = createRoot(rootEl!);
root.render(React.createElement(Playground));
