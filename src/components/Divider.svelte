<script lang="ts">
  export let container: HTMLElement;
  export let leftWidth = 50;

  export let minWidth = 25;

  export let isGrabbing = false;

  const handleMouseDown = (e: MouseEvent) => {
    isGrabbing = true;

    e.preventDefault();
  };

  const handleMouseUp = () => {
    isGrabbing = false;
  };

  const handleMouseMove = (e: MouseEvent) => {
    if (!isGrabbing) return;

    const position = e.pageX;

    const newLeftWidth =
      ((position - container.clientLeft) / container.offsetWidth) * 100;

    leftWidth = Math.max(minWidth, Math.min(100 - minWidth, newLeftWidth));
  };
</script>

<svelte:window on:mousemove={handleMouseMove} on:mouseup={handleMouseUp} />

<div class="divider" on:mousedown={handleMouseDown} aria-hidden="true" />

<style lang="scss">
  .divider {
    background-color: var(--secondary-color);
    transition: background-color 0.5s;
    opacity: 0.75;
    cursor: ew-resize;

    &:hover {
      background-color: var(--tertiary-color);
    }
  }
</style>
