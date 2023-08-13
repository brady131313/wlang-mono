const main = (bindings) => {
  console.log("WASM Running", Object.keys(bindings));
  const { parseTree } = bindings;

  const input = document.querySelector("#input");
  const cstOut = document.querySelector("#cst");
  const hirOut = document.querySelector("#hir");
  const formattedOut = document.querySelector("#formatted");

  const onInput = (input) => {
    try {
      const treeStr = parseTree(input);
      cstOut.textContent = treeStr;
      console.log("ok");
    } catch (e) {
      console.error(e);
    }
  };

  onInput(input.value);
  input.addEventListener("input", (e) => onInput(e.target.value));
};
