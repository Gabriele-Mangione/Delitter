<script lang="ts">
  /** Pixels to reserve above the map (e.g., navbar height) */
  export let top = 0;

  /** Pixels to reserve below the map (e.g., bottom dock height) */
  export let bottom = 0;

  /** Optional extra classes */
  export let className = '';

  // expose the root for parents that want to observe it
  export let el: HTMLDivElement;
</script>

<div
  bind:this={el}
  class={`relative overflow-hidden ${className}`}
  style="
    height: calc(
      100vh
      - var(--top, 0px)
      - var(--bottom, 0px)
      - env(safe-area-inset-top, 0px)
      - env(safe-area-inset-bottom, 0px)
    );
    --top: {top}px;
    --bottom: {bottom}px;
  "
>
  <slot />
</div>
