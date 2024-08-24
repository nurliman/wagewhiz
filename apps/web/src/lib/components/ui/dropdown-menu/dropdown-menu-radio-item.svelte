<script lang="ts">
  import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui";
  import DotFilled from "svelte-radix/DotFilled.svelte";
  import { cn } from "$lib/utils/shadcn.ts";

  type $$Props = DropdownMenuPrimitive.RadioItemProps;
  type $$Events = DropdownMenuPrimitive.RadioItemEvents;

  let className: $$Props["class"] = undefined;
  export let value: DropdownMenuPrimitive.RadioItemProps["value"];
  export { className as class };
</script>

<DropdownMenuPrimitive.RadioItem
  class={cn(
    "data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground relative flex cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
    className,
  )}
  {value}
  {...$$restProps}
  on:click
  on:keydown
  on:focusin
  on:focusout
  on:pointerdown
  on:pointerleave
  on:pointermove
>
  <span class="flex-center absolute left-2 size-3.5">
    <DropdownMenuPrimitive.RadioIndicator>
      <DotFilled class="size-4 fill-current" />
    </DropdownMenuPrimitive.RadioIndicator>
  </span>
  <slot />
</DropdownMenuPrimitive.RadioItem>
