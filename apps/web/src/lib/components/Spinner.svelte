<script lang="ts">
  import { cn } from "$lib/utils/shadcn.ts";

  type Color = "primary" | "secondary" | "accent";

  export let containerClass: string = "";
  export let color: Color = "primary";
</script>

<div class={cn("flex-center size-12", containerClass)}>
  <div class={cn(`the-spinner the-spinner-${color}`, $$restProps.class)}></div>
</div>

<style lang="postcss">
  .the-spinner {
    @apply m-auto block size-full;
    @apply rounded-full border-[6px];
    @apply animate-[spin_.5s_infinite_linear];

    &-primary {
      @apply border-primary/20 border-t-primary;
    }

    &-secondary {
      @apply border-secondary/20 border-t-secondary;
    }

    &-accent {
      @apply border-accent/20 border-t-accent;
    }
  }
</style>
