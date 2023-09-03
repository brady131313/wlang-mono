import { JSCompletionTrie } from "wlang-web";

export const getCursorOffsetInContentEditable = (element: HTMLElement): number | null => {
    let selection: Selection | null = window.getSelection();
    if (!selection || selection.rangeCount < 1) return null;

    let range = selection.getRangeAt(0);
    let preCaretRange = range.cloneRange();
    let treeWalker, currentNode: Node | null;
    let charCount = 0;

    preCaretRange.selectNodeContents(element);
    preCaretRange.setEnd(range.startContainer, range.startOffset);
    treeWalker = document.createTreeWalker(preCaretRange.commonAncestorContainer, NodeFilter.SHOW_TEXT, null);

    while ((currentNode = treeWalker.nextNode())) {
        charCount += currentNode.nodeValue ? currentNode.nodeValue.length : 0;
    }

    // Add newline characters for nested divs
    let walker = document.createTreeWalker(element, NodeFilter.SHOW_ELEMENT, null);
    while ((currentNode = walker.nextNode())) {
        if (currentNode.nodeName === 'DIV') {
            charCount++;
        }
    }

    return charCount;
}

const seed_exercises: string[] = [
    "bench press",
    "back squat",
    "deadlift",
    "front squat"
]

export const COMPLETION = new JSCompletionTrie()
seed_exercises.forEach(e => COMPLETION.addExercise(e))