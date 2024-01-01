<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import FileIcon from "./icons/File.svelte";
  import PlusIcon from "./icons/Plus.svelte";
  import SaveIcon from "./icons/Save.svelte";

  interface Events {
    newFile: {};
    selectFile: {};
    saveFile: {};
  }

  const dispatch = createEventDispatcher<Events>();
</script>

<section class="bar" data-tauri-drag-region>
  <button class="button" on:click={() => dispatch("selectFile", {})}>
    <FileIcon fill="var(--tertiary-color)" />
  </button>
  <button class="button" on:click={() => dispatch("newFile", {})}>
    <PlusIcon fill="var(--tertiary-color)" stroke="var(--tertiary-color)" />
  </button>
  <button class="button" on:click={() => dispatch("saveFile", {})}>
    <SaveIcon />
  </button>
</section>

<style lang="scss">
  .bar {
    display: flex;
    gap: 0.75rem;
  }

  .button {
    width: 1.3rem;
    height: 1.3rem;

    @media (hover: hover) {
      transition: transform 0.2s;

      &:hover {
        transform: scale(0.9);
      }
    }
  }
</style>
