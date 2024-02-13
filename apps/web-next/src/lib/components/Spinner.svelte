<script lang="ts">
  import { clsx } from "clsx";
  type Color = "teal";
  export let containerClass: string = "";
  export let color: Color = "teal";
</script>

<div class={clsx("flex justify-center items-center h-12 w-12 animate-spin", containerClass)}>
  <div class={clsx(`the-spinner the-spinner-${color}`, $$restProps.class)}></div>
</div>

<style lang="postcss">
  .the-spinner {
    display: block;
    margin: auto;
    height: 100%;
    width: 100%;
    border-width: 6px;
    border-radius: 50%;
    animation: rotation 0.6s infinite linear;

    &-teal {
      @apply border-teal-200 border-t-teal;
    }
  }

  @keyframes rotation {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(359deg);
    }
  }
</style>
