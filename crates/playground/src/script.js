const main = (bindings) => {
  console.log("WASM Running", Object.keys(bindings));
  const { parseTree } = bindings;

  const input = document.querySelector("#input");
  const cstOut = document.querySelector("#cst");
  const hirOut = document.querySelector("#hir");
  const formattedOut = document.querySelector("#formatted");

  const onInput = (input) => {
    try {
      const { cst_str, formatted_str } = parseTree(input);
      cstOut.textContent = cst_str;
      formattedOut.innerHTML = formatted_str;
      console.log("ok");
    } catch (e) {
      console.error(e);
    }
  };

  onInput(input.value);
  input.addEventListener("input", (e) => onInput(e.target.value));
};
