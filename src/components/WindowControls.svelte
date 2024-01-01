<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { appWindow } from "@tauri-apps/api/window";

  import Icon from "@iconify/svelte";

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
    <Icon class="icon" icon="mdi:window-minimize" />
  </button>
  <button class="button" on:click={appWindow.toggleMaximize}>
    {#if isMaximixed}
      <Icon class="icon" icon="mdi:window-restore" />
    {:else}
      <Icon class="icon" icon="mdi:window-maximize" />
    {/if}
  </button>
  <button class="button" on:click={appWindow.close}>
    <Icon class="icon" icon="mdi:window-close" />
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
