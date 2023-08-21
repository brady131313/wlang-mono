const main = (bindings) => {
  console.log("WASM Running", Object.keys(bindings));
  const { WorkoutCst, WorkoutHir } = bindings;

  const input = document.querySelector("#input");
  const cstOut = document.querySelector("#cst");
  const hirOut = document.querySelector("#hir");
  const formattedOut = document.querySelector("#formatted");

  const onInput = (input) => {
    try {
      const cst = new WorkoutCst(input);
      cstOut.textContent = cst.toString();
      formattedOut.innerHTML = cst.formattedString();
      console.log(cst.errors);

      const hir = new WorkoutHir(cst);
      hirOut.textContent = hir.toString();
    } catch (e) {
      console.error(e);
    }
  };

  onInput(input.value);
  input.addEventListener("input", (e) => onInput(e.target.value));
};
