import { WorkoutCst, WorkoutHir } from "playground/playground";
import React, { useEffect, useState } from "react";

const BORDER_CLASS = "border border-gray-400 rounded-xl flex-grow p-3";

const GridItem: React.FC<
  { title: string; content?: string; children?: React.ReactNode }
> = (
  { title, content, children },
) => (
  <div className="flex flex-col gap-y-3">
    <h1 className="text-2xl font-bold tracking-wide">{title}</h1>
    {!children
      ? (
        <div className={`${BORDER_CLASS} overflow-y-auto`}>
          <p className="whitespace-pre">{content}</p>
        </div>
      )
      : children}
  </div>
);

function Playground() {
  const [input, setInput] = useState("");
  const [cst, setCst] = useState<WorkoutCst | null>(null);
  const [hir, setHir] = useState<WorkoutHir | null>(null);

  useEffect(() => {
    const newCst = new WorkoutCst(input);
    setCst(newCst);

    const newHir = new WorkoutHir(newCst);
    setHir(newHir);
  }, [input, setCst]);

  return (
    <main className="min-h-screen max-h-screen w-full grid grid-cols-2 grid-rows-2 p-8 gap-8">
      <GridItem title="Workout Input">
        <textarea
          className={`${BORDER_CLASS} resize-none focus:outline-none focus:ring-4 focus:ring-gray-300/70`}
          value={input}
          onChange={(e) => setInput(e.target.value)}
        />
      </GridItem>
      <GridItem title="Formatted">
        <div
          className={`${BORDER_CLASS} overflow-y-auto whitespace-pre workout`}
          dangerouslySetInnerHTML={{ __html: cst?.formattedString() || "" }}
        />
      </GridItem>
      <GridItem title="CST" content={cst?.toString()} />
      <GridItem title="HIR" content={hir?.toString()} />
    </main>
  );
}

export default Playground;
