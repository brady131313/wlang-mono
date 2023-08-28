import { WorkoutCst, WorkoutHir } from "playground/playground";
import React, { useState } from "react";
import Editor from "./Editor";
import { JSTokenContext } from "playground";

const BORDER_CLASS = "border border-gray-400 rounded-xl flex-grow p-3";

const GridItem: React.FC<
  { title: string; content?: string; children?: React.ReactNode }
> = (
  { title, content, children },
) => (
    <div className="flex flex-col gap-y-3 relative">
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
  const [context, setContext] = useState<JSTokenContext | null>(null);
  const [completions, setCompletions] = useState<string[]>([]);
  const [cst, setCst] = useState<WorkoutCst | null>(null);
  const [hir, setHir] = useState<WorkoutHir | null>(null);

  const handleInput = (input: string, offset?: number) => {
    setInput(input);

    const newCompletions = [];
    if (input.charAt(0) == "n") {
      newCompletions.push("nice");
    }
    setCompletions(newCompletions);

    const newCst = new WorkoutCst(input);
    setCst(newCst);

    if (offset) {
      const lookup = newCst.lookupOffset(offset);
      setContext(lookup || null)
    }

    const newHir = new WorkoutHir(newCst);
    setHir(newHir);
  };

  return (
    <main className="min-h-screen max-h-screen w-full grid grid-cols-2 grid-rows-2 p-8 gap-8">
      <GridItem title="Workout Input">
        <Editor
          value={input}
          onEdit={handleInput}
          showMenu={completions.length > 0}
        >
          <ul className="bg-gray-50 p-1.5 rounded divide-y divide-gray-200">
            {completions.map((c) => <li key={c}>{c}</li>)}
          </ul>
        </Editor>
        {context && (
          <div className="absolute bottom-2 right-4">
            {context.treeKind && context.treeKind + ">"}
            {context.token.kind}
          </div>
        )}
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
