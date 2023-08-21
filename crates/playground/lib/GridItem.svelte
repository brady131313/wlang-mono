<script lang="ts">
  export let title: string
  export let content: string = ""

  let borderClass = "border border-gray-400 rounded-xl flex-grow p-3"
</script>

<div class="flex flex-col gap-y-3">
  <h1 class="text-2xl font-bold tracking-wide">{title}</h1>
  <slot {borderClass}>
    <div class={`${borderClass} overflow-y-auto`}>
      <p class="whitespace-pre">{content}</p>
    </div>
  </slot>
</div>
