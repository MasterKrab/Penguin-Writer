<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import IconFile from "~icons/mdi/file";
  import IconFilePlus from "~icons/mdi/file-plus";
  import IconContentSave from "~icons/mdi/content-save";

  interface Events {
    newFile: {};
    selectFile: {};
    saveFile: {};
  }

  const dispatch = createEventDispatcher<Events>();
</script>

<section class="bar" data-tauri-drag-region>
  <button class="button" on:click={() => dispatch("selectFile", {})}>
    <IconFile class="icon" />
  </button>
  <button class="button" on:click={() => dispatch("newFile", {})}>
    <IconFilePlus class="icon" />
  </button>
  <button class="button" on:click={() => dispatch("saveFile", {})}>
    <IconContentSave class="icon" />
  </button>
</section>

<style lang="scss">
  .bar {
    display: flex;
    gap: 0.5rem;
  }

  :global(.icon) {
    width: 1.5rem;
    height: 1.5rem;
    vertical-align: middle;

    @media (hover: hover) {
      transition: color 0.2s;
    }
  }

  .button {
    width: 1.75rem;
    height: 1.75rem;
    border-radius: 0.25rem;

    @media (hover: hover) {
      transition: background-color 0.2s;

      &:hover {
        background-color: var(--tertiary-color);

        & :global(.icon) {
          color: var(--primary-color);
        }
      }
    }
  }
</style>
