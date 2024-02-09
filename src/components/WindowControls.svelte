<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { appWindow } from "@tauri-apps/api/window";

  import IconWindowMinimize from "~icons/mdi/window-minimize";
  import IconWindowMaximize from "~icons/mdi/window-maximize";
  import IconWindowRestore from "~icons/mdi/window-restore";
  import IconWindowClose from "~icons/mdi/window-close";

  let isMaximixed: boolean;
  let interval: number;

  const updateIsMaximized = async () => {
    isMaximixed = await appWindow.isMaximized();
  };

  onMount(() => {
    updateIsMaximized();

    interval = setInterval(updateIsMaximized, 0);
  });

  onDestroy(() => {
    clearInterval(interval);
  });
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<section class="top" on:click={updateIsMaximized}>
  <button class="button" on:click={appWindow.minimize}>
    <IconWindowMinimize class="icon" />
  </button>
  <button class="button" on:click={appWindow.toggleMaximize}>
    {#if isMaximixed}
      <IconWindowRestore class="icon" />
    {:else}
      <IconWindowMaximize class="icon" />
    {/if}
  </button>
  <button class="button" on:click={appWindow.close}>
    <IconWindowClose class="icon" />
  </button>
</section>

<style lang="scss">
  .top {
    display: flex;
    align-items: center;
    gap: 0.75rem;
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
    display: grid;
    place-items: center;
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
