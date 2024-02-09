<script lang="ts">
  import { HighlightAuto } from "svelte-highlight";
  import dracula from "svelte-highlight/styles/dracula";

  export let text: string;
</script>

<svelte:head>
  {@html dracula}
</svelte:head>

<HighlightAuto code={text} />
