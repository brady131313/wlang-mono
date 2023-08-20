import { useEffect } from "react";
import init, { WorkoutCst } from "@rsw/weightroom";

function App() {
  useEffect(() => {
    const loadWasm = async () => {
      await init();
    };

    loadWasm();
  });

  const parseInput = (input: string) => {
    try {
      const cst = new WorkoutCst(input);
      console.log(cst.toString());
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div className="card">
      <textarea onChange={(e) => parseInput(e.target.value)} />
    </div>
  );
}

export default App;
