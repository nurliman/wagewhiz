<script lang="ts">
  import { clsx } from "clsx";
  import type { EmployeeStatus } from "$lib/types.ts";

  export let status: EmployeeStatus;
</script>

<div
  {...$$restProps}
  class={clsx(
    "badge variant-ghost",
    {
      "status-badge-permanent": status === "Permanent",
      "status-badge-contract": status === "Contract",
      "status-badge-probation": status === "Probation",
    },
    $$restProps.class,
  )}
>
  {status}
</div>

<style lang="postcss">
  .status-badge {
    &-permanent {
      @apply ring-[1px] ring-green-500/80 dark:ring-green-500/80 ring-inset;
      @apply bg-green-500/20 dark:bg-green-500/20;
    }

    &-contract {
      @apply ring-[1px] ring-yellow-500/80 dark:ring-yellow-500/80 ring-inset;
      @apply bg-yellow-500/20 dark:bg-yellow-500/20;
    }

    &-probation {
      @apply ring-[1px] ring-purple-500/80 dark:ring-purple-500/80 ring-inset;
      @apply bg-purple-500/20 dark:bg-purple-500/20;
    }
  }
</style>
