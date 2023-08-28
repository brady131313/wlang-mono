import { WorkoutCst, WorkoutHir, JSTokenContext, JSCompletionTrie } from "playground/playground";
import React, { useState } from "react";
import Editor from "./Editor";

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

const COMPLETION = new JSCompletionTrie()
COMPLETION.add_exercise('DB Bench Press')
COMPLETION.add_exercise('DB Incline Press')
COMPLETION.add_exercise('DB Shoulder Press')
COMPLETION.add_exercise('DB Row')
COMPLETION.add_exercise('DB Curl')

function Playground() {
  const [input, setInput] = useState("");
  const [context, setContext] = useState<JSTokenContext | null>(null);
  const [completions, setCompletions] = useState<string[]>([]);
  const [cst, setCst] = useState<WorkoutCst | null>(null);
  const [hir, setHir] = useState<WorkoutHir | null>(null);

  const handleInput = (input: string, offset?: number) => {
    setInput(input);

    const newCst = new WorkoutCst(input);
    setCst(newCst);

    const newCompletions: string[] = [];
    if (offset) {
      const lookup = newCst.lookupOffset(offset);
      setContext(lookup || null)

      if (lookup && lookup.treeKind === 'exercise') {
        const exercise = lookup.token.kind === 'ident' ? input.substring(lookup.token.start, offset) : ""
        console.log(COMPLETION.complete_exercise(exercise))
      }
    }
    setCompletions(newCompletions);

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
