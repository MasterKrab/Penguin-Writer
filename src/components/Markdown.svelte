<script lang="ts">
  import { invoke } from "@tauri-apps/api";

  export let value: string;
</script>

{#await invoke("markdown_to_html", { value }) then html}
  <div class="markdown">
    {@html html}
  </div>
{/await}

<style lang="scss" global>
  .markdown {
    padding: 0.5rem 1rem;
    color: #fff;

    * {
      margin-top: 0.25em;
      margin-bottom: 0.25em;
    }

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

    pre {
      background-color: var(--cuaternary-color);
      padding: 1rem;
    }
  }
</style>
