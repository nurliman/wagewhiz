<script lang="ts">
  import { Command as CommandPrimitive } from "cmdk-sv";
  import { cn } from "$lib/utils/shadcn.ts";

  type $$Props = CommandPrimitive.CommandProps;

  export let value: $$Props["value"] = undefined;

  let className: string | undefined | null = undefined;
  export { className as class };
</script>

<CommandPrimitive.Root
  class={cn(
    "bg-popover text-popover-foreground flex size-full flex-col overflow-hidden rounded-md",
    className,
  )}
  bind:value
  {...$$restProps}
>
  <slot />
</CommandPrimitive.Root>
