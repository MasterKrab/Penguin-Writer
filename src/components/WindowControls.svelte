<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { appWindow } from "@tauri-apps/api/window";

  import LineIcon from "./icons/Line.svelte";
  import RestoreWindowIcon from "./icons/RestoreWindow.svelte";
  import MaximizeIcon from "./icons/Maximize.svelte";
  import CloseIcon from "./icons/Close.svelte";

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
    <LineIcon fill="var(--tertiary-color)" />
  </button>
  <button class="button" on:click={appWindow.toggleMaximize}>
    {#if isMaximixed}
      <RestoreWindowIcon fill="var(--tertiary-color)" />
    {:else}
      <MaximizeIcon fill="var(--tertiary-color)" />
    {/if}
  </button>
  <button class="button" on:click={appWindow.close}>
    <CloseIcon fill="var(--tertiary-color)" />
  </button>
</section>

<style lang="scss">
  .top {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .button {
    display: grid;
    place-items: center;
    width: 1rem;
    height: 1rem;
  }
</style>
