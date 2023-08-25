<script lang="ts">
  import { clsx } from "clsx";
  type Variant = "primary" | "secondary" | "tertiary";
  export let containerClass: string = "";
  export let variant: Variant = "secondary";
</script>

<div class={clsx("flex justify-center items-center h-12 w-12 animate-spin", containerClass)}>
  <div class={clsx(`the-spinner the-spinner-${variant}`, $$restProps.class)}></div>
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
  }

  .the-spinner-primary {
    @apply border-primary-900 border-t-primary-500;
  }

  .the-spinner-secondary {
    @apply border-secondary-900 border-t-secondary-500;
  }

  .the-spinner-tertiary {
    @apply border-tertiary-900 border-t-tertiary-500;
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
