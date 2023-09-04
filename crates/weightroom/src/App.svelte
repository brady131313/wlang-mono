<script lang="ts">
    import Container from './lib/Container.svelte'
    import Card from './lib/Card.svelte'
    import TokenPill from './lib/TokenPill.svelte'
    import {
        type JSToken,
        lex,
        WorkoutCst,
        WorkoutHir,
        type JSTokenContext,
    } from 'wlang-web'
    import { WORKOUTS } from './lib/utils'
    import Editor from './lib/Editor.svelte'

    let input: string = ''
    let offset: number

    let tokens: JSToken[] = []
    let cst: WorkoutCst | null = null
    let context: JSTokenContext | null = null
    let hir: WorkoutHir | null = null

    $: {
        tokens = lex(input)

        cst?.free()
        cst = new WorkoutCst(input)

        if (cst) {
            context = cst.lookupOffset(offset) || null

            hir?.free()
            hir = new WorkoutHir(cst)
        }
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
                        {#each Object.keys(WORKOUTS) as sample}
                            <button
                                class="btn btn-secondary btn-xs"
                                on:click={() => {
                                    input = WORKOUTS[sample].trim()
                                    offset = input.length
                                }}
                            >
                                {sample}
                            </button>
                        {/each}
                    </div>
                    <Editor />
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
                    CST
                </h1>
                <pre class="flex-1 text-gray-700">{cst?.toString()}</pre>
            </Card>
            <Card>
                <h1 slot="header" class="text-lg font-medium text-gray-900">
                    HIR
                </h1>
                <pre class="flex-1 text-gray-700">{hir?.toString()}</pre>
            </Card>
        </div>
    </Container>
</main>
