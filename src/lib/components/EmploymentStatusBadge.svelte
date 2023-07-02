<script lang="ts">
  import type { EmployeeStatus } from "$lib/types.ts";
  import type { BadgeProps } from "flowbite-svelte/Badge.svelte";
  import Badge from "flowbite-svelte/Badge.svelte";

  type StatusColors = Record<EmployeeStatus, BadgeProps["color"]>;

  const STATUS_COLORS: Readonly<StatusColors> = {
    Permanent: "green",
    Contract: "yellow",
    Probation: "purple",
  };

  interface $$Props extends BadgeProps {
    status: EmployeeStatus;
  }

  export let status: EmployeeStatus;
  export let color: BadgeProps["color"] = STATUS_COLORS[status];
  export let border: BadgeProps["border"] = true;
  export let large: BadgeProps["large"] = false;
</script>

<Badge {...$$restProps} {border} {color} {large}>
  {status}
</Badge>
