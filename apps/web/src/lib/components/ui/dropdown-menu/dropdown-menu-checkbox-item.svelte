<script lang="ts">
  import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui";
  import Check from "svelte-radix/Check.svelte";
  import { cn } from "$lib/utils/shadcn.ts";

  type $$Props = DropdownMenuPrimitive.CheckboxItemProps;
  type $$Events = DropdownMenuPrimitive.CheckboxItemEvents;

  let className: $$Props["class"] = undefined;
  export let checked: $$Props["checked"] = undefined;
  export { className as class };
</script>

<DropdownMenuPrimitive.CheckboxItem
  bind:checked
  class={cn(
    "data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground relative flex cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
    className,
  )}
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
    <DropdownMenuPrimitive.CheckboxIndicator>
      <Check class="size-4" />
    </DropdownMenuPrimitive.CheckboxIndicator>
  </span>
  <slot />
</DropdownMenuPrimitive.CheckboxItem>
