<script lang="ts">
    import Container from './lib/Container.svelte'
    import Card from './lib/Card.svelte'
    import Editor from './lib/Editor.svelte'
    import TokenPill from './lib/TokenPill.svelte'
    import { type JSToken, lex } from 'wlang-web'

    let input: string = ''
    let tokens: JSToken[] = []
    let offset: number

    const getSuggestion = () => {
        const word = input.split(' ').pop()
        if (word?.startsWith('a')) return 'apple'.substring(word.length)
        return null
    }

    $: tokens = lex(input)
</script>

<main class="py-4">
    <Container>
        <div class="space-y-8">
            <Card>
                <h1 slot="header" class="text-lg font-medium text-gray-900">
                    Input {offset}
                </h1>

                <Editor
                    bind:value={input}
                    bind:cursorOffset={offset}
                    {getSuggestion}
                />
            </Card>

            <Card>
                <h1 slot="header" class="text-lg font-medium text-gray-900">
                    Tokens
                </h1>
                <div class="flex gap-2">
                    {#each tokens as token}
                        <TokenPill {token} />
                    {/each}
                </div>
            </Card>
        </div>
    </Container>
</main>
