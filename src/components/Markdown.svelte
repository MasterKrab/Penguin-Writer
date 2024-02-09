<script lang="ts">
  import SvelteMarkdown from "svelte-markdown";
  import MarkdownCode from "./MarkdownCode.svelte";

  export let value: string;
</script>

<div class="markdown">
  <SvelteMarkdown source={value} renderers={{ code: MarkdownCode }} />
</div>

<style lang="scss" global>
  .markdown {
    padding: 0.5em 1em;

    h1 {
      border-bottom: 1px solid #ccc;
    }

    blockquote {
      margin-top: 0.5rem;
      margin-bottom: 0.5rem;
    }

    blockquote {
      background-color: rgba(204, 204, 204, 0.5);
      border-left: 3px solid #ccc;
      padding-left: 1rem;
      margin-left: 0;
    }
  }
</style>
