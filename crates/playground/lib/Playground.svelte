<script lang="ts">
  import { WorkoutCst, WorkoutHir } from "playground"
  import GridItem from "./GridItem.svelte";

  let input = ""
  let cst: null | WorkoutCst = null
  let hir: null | WorkoutHir = null

  $: {
    cst?.free()
    cst = new WorkoutCst(input)
  }
</script>

<main class="min-h-screen max-h-screen w-full grid grid-cols-2 grid-rows-2 p-8 gap-8">
  <GridItem title="Workout Input" let:borderClass>
    <textarea 
      bind:value={input}
      class={`${borderClass} resize-none focus:outline-none focus:ring-4 focus:ring-gray-300/70`} 
    />
  </GridItem>
  <GridItem title="Formatted" content={cst?.formattedString() || ""} />
  <GridItem title="CST" content={cst?.toString() || ""} />
  <GridItem title="HIR" content={hir?.toString() || ""} />
</main>
