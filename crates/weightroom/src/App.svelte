<script lang="ts">
    import Container from './lib/Container.svelte'
    import Card from './lib/Card.svelte'
    import Editor from './lib/Editor.svelte'
    import TokenPill from './lib/TokenPill.svelte'
    import {
        type JSToken,
        lex,
        WorkoutCst,
        WorkoutHir,
        type JSTokenContext,
    } from 'wlang-web'
    import { COMPLETION } from './lib/utils'

    let input: string = ''
    let offset: number

    let tokens: JSToken[] = []
    let cst: WorkoutCst | null = null
    let context: JSTokenContext | null = null
    let hir: WorkoutHir | null = null

    const getSuggestion = () => {
        if (context) {
            const lookup = input.substring(
                context.token.start,
                context.token.end
            )
            const result = COMPLETION.completeExercise(lookup)
            const suggestion = result.pop()
            if (suggestion) {
                return suggestion.substring(lookup.length)
            }
        }
        return null
    }

    const SAMPLES: Record<string, string> = {
        simple: `# Bench Press
225 x5 
        `,
        complex: '',
    }

    $: tokens = lex(input)
    $: cst = new WorkoutCst(input)
    $: if (cst) {
        context = cst.lookupOffset(offset) || null
        hir = new WorkoutHir(cst)
    }
</script>

<main class="py-4">
    <Container>
        <div class="space-y-8">
            <Card>
                <h1 slot="header" class="text-lg font-medium text-gray-900">
                    Input {offset}
                </h1>

                <div class="space-y-2">
                    <div class="flex gap-2">
                        {#each Object.keys(SAMPLES) as sample}
                            <button
                                class="btn btn-xs btn-secondary"
                                on:click={() => {
                                    input = SAMPLES[sample].trim()
                                    offset = input.length
                                }}
                            >
                                {sample}
                            </button>
                        {/each}
                    </div>
                    <Editor
                        bind:value={input}
                        bind:cursorOffset={offset}
                        {getSuggestion}
                    />
                    <div class="flex justify-end">
                        {#if context}
                            <p>
                                {context?.tree_kind &&
                                    context.tree_kind + ' > '}{context?.token
                                    .kind}
                            </p>
                        {/if}
                    </div>
                </div>
            </Card>

            <Card>
                <h1 slot="header" class="text-lg font-medium text-gray-900">
                    Tokens
                </h1>
                <div class="flex flex-wrap gap-2">
                    {#each tokens as token}
                        <TokenPill {token} />
                    {/each}
                </div>
            </Card>

            <Card>
                <h1 slot="header" class="text-lg font-medium text-gray-900">
                    Trees
                </h1>
                <div class="flex justify-between gap-4">
                    <pre class="flex-1 text-gray-700">{cst?.toString()}</pre>
                    <pre class="flex-1 text-gray-700">{hir?.toString()}</pre>
                </div>
            </Card>
        </div>
    </Container>
</main>
