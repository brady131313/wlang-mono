import { JSCompletionTrie } from 'wlang-web'

export const getCursorOffsetInContentEditable = (
    element: HTMLElement
): number | null => {
    let selection: Selection | null = window.getSelection()
    if (!selection || selection.rangeCount < 1) return null

    let range = selection.getRangeAt(0)
    let preCaretRange = range.cloneRange()
    let treeWalker, currentNode: Node | null
    let charCount = 0

    preCaretRange.selectNodeContents(element)
    preCaretRange.setEnd(range.startContainer, range.startOffset)
    treeWalker = document.createTreeWalker(
        preCaretRange.commonAncestorContainer,
        NodeFilter.SHOW_TEXT,
        null
    )

    while ((currentNode = treeWalker.nextNode())) {
        charCount += currentNode.nodeValue ? currentNode.nodeValue.length : 0
    }

    // Add newline characters for nested divs
    // let walker = document.createTreeWalker(
    //     element,
    //     NodeFilter.SHOW_ELEMENT,
    //     null
    // )
    // while ((currentNode = walker.nextNode())) {
    //     if (currentNode.nodeName === 'DIV') {
    //         charCount++
    //     }
    // }

    return charCount
}

export const setCursorPosition = (element: HTMLElement, offset: number) => {
    const range = document.createRange()
    const selection = window.getSelection()

    let charCount = 0
    let node

    for (node of element.childNodes) {
        if (node.nodeType === Node.TEXT_NODE) {
            const nextCharCount = charCount + node.textContent!.length
            if (nextCharCount >= offset) {
                break
            }

            charCount = nextCharCount
        } else if (node.nodeName === 'BR') {
            charCount++
            if (charCount >= offset) {
                break
            }
        }
    }

    range.setStart(node!, offset - charCount)
    range.collapse(true)

    selection?.removeAllRanges()
    selection?.addRange(range)
}

const seed_exercises: string[] = [
    'bench press',
    'back squat',
    'deadlift',
    'front squat',
]

export const COMPLETION = new JSCompletionTrie()
seed_exercises.forEach(e => COMPLETION.addExercise(e))
