import { useEffect, useState } from "react";
import init, { add, greet } from "@rsw/weightroom";

function App() {
  const [count, setCount] = useState(0);

  useEffect(() => {
    (async () => {
      await init();
    })();
  }, []);

  return (
    <div className="card">
      <button onClick={() => setCount(add(5, 10))}>
        count is {count}
      </button>
    </div>
  );
}

export default App;
