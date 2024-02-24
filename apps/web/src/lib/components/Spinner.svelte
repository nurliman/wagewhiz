<script lang="ts">
  import { cn } from "$lib/utils/shadcn";

  type Color = "teal";

  export let containerClass: string = "";
  export let color: Color = "teal";
</script>

<div class={cn("flex h-12 w-12 items-center justify-center", containerClass)}>
  <div class={cn(`the-spinner the-spinner-${color}`, $$restProps.class)}></div>
</div>

<style lang="postcss">
  .the-spinner {
    @apply m-auto block h-full w-full;
    @apply rounded-full border-[6px];
    @apply animate-[spin_.5s_infinite_linear];

    &-teal {
      @apply border-t-teal border-teal-200;
      @apply dark:border-teal-900 dark:border-t-teal-500;
    }
  }
</style>
