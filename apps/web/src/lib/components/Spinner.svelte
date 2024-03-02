<script lang="ts">
  import { cn } from "$lib/utils/shadcn";

  type Color = "primary" | "secondary" | "accent";

  export let containerClass: string = "";
  export let color: Color = "secondary";
</script>

<div class={cn("flex h-12 w-12 items-center justify-center", containerClass)}>
  <div class={cn(`the-spinner the-spinner-${color}`, $$restProps.class)}></div>
</div>

<style lang="postcss">
  .the-spinner {
    @apply m-auto block h-full w-full;
    @apply rounded-full border-[6px];
    @apply animate-[spin_.5s_infinite_linear];

    &-primary {
      @apply border-t-primary border-primary/20;
    }

    &-secondary {
      @apply border-t-secondary border-secondary/20;
    }

    &-accent {
      @apply border-t-accent border-accent/20;
    }
  }
</style>
