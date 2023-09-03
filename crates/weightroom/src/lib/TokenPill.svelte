<script lang="ts">
    import type { JSToken } from 'wlang-web'

    export let token: JSToken
</script>

<div class="flex items-baseline gap-x-1 rounded-md bg-gray-100 px-1 py-0.5">
    <span class="text-sm font-medium text-gray-800">{token.kind}</span>
    <span class="text-xs text-gray-600">{token.start}..{token.end}</span>
</div>
