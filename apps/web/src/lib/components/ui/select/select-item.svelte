<script lang="ts">
  import { Select as SelectPrimitive } from "bits-ui";
  import Check from "svelte-radix/Check.svelte";
  import { cn } from "$lib/utils/shadcn.ts";

  type $$Props = SelectPrimitive.ItemProps;
  type $$Events = Required<SelectPrimitive.ItemEvents>;

  let className: $$Props["class"] = undefined;
  export let value: $$Props["value"];
  export let label: $$Props["label"] = undefined;
  export let disabled: $$Props["disabled"] = undefined;
  export { className as class };
</script>

<SelectPrimitive.Item
  {value}
  {disabled}
  {label}
  class={cn(
    "data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-2 pr-8 text-sm outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
    className,
  )}
  {...$$restProps}
  on:click
  on:pointermove
  on:focusin
>
  <span class="flex-center absolute right-2 size-3.5">
    <SelectPrimitive.ItemIndicator>
      <Check class="size-4" />
    </SelectPrimitive.ItemIndicator>
  </span>
  <slot>
    {label || value}
  </slot>
</SelectPrimitive.Item>
