<script lang="ts">
  import type Tab from "types/tab";

  import { createEventDispatcher } from "svelte";

  import MardownIcon from "./icons/Mardown.svelte";
  import PlusIcon from "./icons/Plus.svelte";
  import CloseIcon from "./icons/Close.svelte";

  export let tabs: Tab[] = [];
  export let selectedTabIndex: number = 0;

  interface TabData {
    index: number;
  }

  const dispatch = createEventDispatcher<{
    selectTab: TabData;
    closeTab: TabData;
    newFile: {};
  }>();

  const selectTab = (index: number) => dispatch("selectTab", { index });

  const closeTab = (index: number) => dispatch("closeTab", { index });

  const handleNewFile = () => dispatch("newFile", {});
</script>

<div class="bar">
  <ul class="tabs" role="tablist">
    {#each tabs as { name, path }, index}
      {@const isSelected = index === selectedTabIndex}

      <li
        class="tab"
        class:tab--active={isSelected}
        aria-selected={isSelected}
        role="tab"
        title={path}
      >
        <button
          class="button"
          on:click|stopPropagation={() => selectTab(index)}
        >
          <MardownIcon
            on:click={() => selectTab(index)}
            fill="var(--tertiary-color)"
            width={20}
          />
          {name}
        </button>
        <button
          class="button-close"
          class:button-close--active={isSelected}
          on:click={() => closeTab(index)}
          aria-label="Close tab {name}"
        >
          <CloseIcon fill="var(--tertiary-color)" />
        </button>
      </li>
    {/each}
  </ul>
  <button class="button-add" on:click={handleNewFile} aria-label="New File">
    <PlusIcon width={13} height={13} fill="var(--tertiary-color)" />
  </button>
</div>

<style lang="scss">
  :global(:root) {
    --tab-bar-height: 1.5rem;
  }

  .bar {
    display: grid;
    grid-template-columns: calc(100% - 1.75rem) 1.75rem;
  }

  .tabs {
    display: flex;
    padding-left: 0;
    margin-top: 0;
    margin-bottom: 0;
    height: var(--tab-bar-height);
    background-color: var(--secondary-color);
    overflow-x: auto;

    &::-webkit-scrollbar {
      height: 0.25rem;
      background-color: var(--primary-color);
    }

    &::-webkit-scrollbar-thumb {
      background-color: var(--secondary-color);
    }

    &:active::-webkit-scrollbar-thumb {
      background-color: var(--tertiary-color);
    }
  }

  .tab {
    display: flex;
    gap: 0.5rem;
    padding-right: 0.3rem;
    transition: background-color 0.2s;

    &--active,
    &:hover {
      background-color: var(--primary-color);
    }
  }

  .button {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    padding-left: 0.75rem;
  }

  .button-close {
    display: grid;
    place-items: center;
    align-self: center;
    width: 1rem;
    height: 1rem;
    padding: 0.25rem;
    transition: transform 0.5s;
    border-radius: 0.25rem;
    transition: background-color 0.15s;

    &:hover {
      background-color: var(--primary-color);
    }

    &--active:hover {
      background-color: var(--secondary-color);
    }
  }

  .button-add {
    transition: background-color 0.15s;

    &:hover {
      background-color: var(--secondary-color);
    }
  }
</style>
