import { ReactNode, useRef, useState } from "react";

type CaretPosition = {
  left: number;
  top: number;
  offset: number;
};

interface EditorProps {
  value: string;
  onEdit: (input: string, offset?: number) => void;
  children?: ReactNode;
  showMenu?: boolean;
}

const getCaretRect = (textarea: HTMLTextAreaElement): CaretPosition | null => {
  if (!textarea) return null;

  const start = textarea.selectionStart;
  const contentBeforeCaret = textarea.value.substring(0, start);
  const contentAfterCaret = textarea.value.substring(start);
  const marker = '<span id="caret-marker">|</span>';

  // Create div at current cursor position, then measure its position
  const mirrorDiv = document.createElement("div");
  mirrorDiv.style.cssText = window.getComputedStyle(textarea).cssText;
  mirrorDiv.style.top = "12px"; // equal to padding
  mirrorDiv.style.left = "12px"; // equal to padding
  mirrorDiv.style.position = "absolute";
  mirrorDiv.style.visibility = "hidden";
  mirrorDiv.style.overflow = "visible";
  mirrorDiv.style.whiteSpace = "pre-wrap";
  mirrorDiv.innerHTML = contentBeforeCaret + marker + contentAfterCaret;

  textarea.parentNode?.appendChild(mirrorDiv);
  const markerSpan = document.getElementById("caret-marker");
  const caretRect = markerSpan?.getBoundingClientRect();
  const textareaRect = textarea.getBoundingClientRect();
  textarea.parentNode?.removeChild(mirrorDiv);

  if (caretRect) {
    return {
      left: caretRect.left - textareaRect.left,
      top: caretRect.top - textareaRect.top - 8,
      offset: start,
    };
  } else {
    return null;
  }
};

function Editor({ value, onEdit, children, showMenu = true }: EditorProps) {
  const inputRef = useRef<HTMLTextAreaElement>(null);
  const [caretVisible, setCaretVisible] = useState(false);
  const [caretPos, setCaretPos] = useState<CaretPosition | null>(null);

  const handleInput = (input: string) => {
    const rect = inputRef.current && getCaretRect(inputRef.current);
    setCaretPos(rect);

    onEdit(input, rect?.offset);
  };

  return (
    <div className="relative flex-grow">
      <textarea
        ref={inputRef}
        value={value}
        onChange={(e) => handleInput(e.target.value)}
        onFocus={() => setCaretVisible(true)}
        onBlur={() => setCaretVisible(true)}
        className="block w-full h-full border border-gray-400 rounded-xl p-3 focus:outline-none focus:ring-4 focus:ring-gray-300/70 resize-none"
      />
      {caretVisible && caretPos && showMenu && (
        <div
          className="absolute -translate-x-1/2 -translate-y-full z-10"
          style={{ left: caretPos?.left, top: caretPos?.top }}
        >
          {children}
        </div>
      )}
    </div>
  );
}

export default Editor;
