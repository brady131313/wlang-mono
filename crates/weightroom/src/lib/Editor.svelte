<script lang="ts">
    import { Composer, ContentEditable, PlainTextPlugin } from 'svelte-lexical'

    const config = {
        nodes: [],
        onError: error => {
            throw error
        },
    }
</script>

<Composer initialConfig={config}>
    <ContentEditable />
    <PlainTextPlugin />
</Composer>
