const main = (bindings) => {
  console.log("WASM Running", Object.keys(bindings));
  const { WorkoutCst } = bindings;

  const input = document.querySelector("#input");
  const cstOut = document.querySelector("#cst");
  const hirOut = document.querySelector("#hir");
  const formattedOut = document.querySelector("#formatted");

  const onInput = (input) => {
    try {
      const cst = new WorkoutCst(input);

      cstOut.textContent = cst.toString();
      hirOut.textContent = cst.hirString();
      formattedOut.innerHTML = cst.formattedString();
    } catch (e) {
      console.error(e);
    }
  };

  onInput(input.value);
  input.addEventListener("input", (e) => onInput(e.target.value));
};
