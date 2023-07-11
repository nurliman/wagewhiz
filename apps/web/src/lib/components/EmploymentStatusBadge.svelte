<script lang="ts">
  import type { EmployeeStatus } from "$lib/types.ts";

  export let status: EmployeeStatus;
</script>

<div
  {...$$restProps}
  class={`badge variant-ghost ` + $$restProps.class}
  class:permanent={status === "Permanent"}
  class:contract={status === "Contract"}
  class:probation={status === "Probation"}
>
  {status}
</div>

<style lang="scss">
  .badge {
    &.permanent {
      @apply ring-[1px] ring-green-500/80 dark:ring-green-500/80 ring-inset;
      @apply bg-green-500/20 dark:bg-green-500/20;
    }

    &.contract {
      @apply ring-[1px] ring-yellow-500/80 dark:ring-yellow-500/80 ring-inset;
      @apply bg-yellow-500/20 dark:bg-yellow-500/20;
    }

    &.probation {
      @apply ring-[1px] ring-purple-500/80 dark:ring-purple-500/80 ring-inset;
      @apply bg-purple-500/20 dark:bg-purple-500/20;
    }
  }
</style>
