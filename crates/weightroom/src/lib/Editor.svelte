<script lang="ts">
    import {
        getCursorOffsetInContentEditable,
        setCursorPosition,
    } from './utils'

    export let value: string // the raw string with no html formatting
    export let cursorOffset = 0
    export let getSuggestion: (() => string | null) | null = null
    export let swipeThreshold = 100

    let inputEl: HTMLDivElement
    let mirroredContent = ''

    let suggestion: string | null

    let startX: number = 0
    let endX: number = 0

    const acceptSuggestion = () => {
        if (suggestion) {
            const newInput =
                value.substring(0, cursorOffset) +
                suggestion +
                value.substring(cursorOffset)
            inputEl.innerText = newInput
            setCursorPosition(inputEl, cursorOffset + suggestion.length)
            cursorOffset = cursorOffset + suggestion.length
        }
    }

    const onKeyDown = (event: KeyboardEvent) => {
        if (event.key === 'Tab' && suggestion) {
            event.preventDefault()
            acceptSuggestion()
        }
    }

    const onTouchStart = (event: TouchEvent) => {
        startX = event.touches[0].clientX
    }

    const onTouchMove = (event: TouchEvent) => {
        endX = event.touches[0].clientX
    }

    const onTouchEnd = () => {
        if (startX - endX > swipeThreshold) {
            // swipe left
        } else if (endX - startX > swipeThreshold) {
            // swipe right
            acceptSuggestion()
        }
    }

    $: if (value) {
        console.log(value)
        cursorOffset = getCursorOffsetInContentEditable(inputEl) || 0
        suggestion = getSuggestion?.() || null

        const preCursorText = value.substring(0, cursorOffset)
        const postCursorText = value.substring(cursorOffset)

        mirroredContent = `${preCursorText}<span>${
            suggestion ?? ''
        }</span>${postCursorText}`
    }
</script>

<div class="relative">
    <div
        bind:this={inputEl}
        bind:innerText={value}
        on:keydown={onKeyDown}
        on:touchstart={onTouchStart}
        on:touchmove={onTouchMove}
        on:touchend={onTouchEnd}
        spellcheck="false"
        role="textbox"
        contenteditable
        tabindex="0"
        class="relative z-20 whitespace-pre rounded-md border-0 p-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-600"
    />

    <div
        class="pointer-events-none absolute inset-0 z-10 whitespace-pre p-1.5 text-gray-400"
        aria-hidden
    >
        {@html mirroredContent}
    </div>
</div>
