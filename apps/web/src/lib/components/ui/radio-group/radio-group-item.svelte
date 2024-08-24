<script lang="ts">
  import { RadioGroup as RadioGroupPrimitive } from "bits-ui";
  import Check from "svelte-radix/Check.svelte";
  import { cn } from "$lib/utils/shadcn.ts";

  type $$Props = RadioGroupPrimitive.ItemProps & {
    value: string;
  };
  type $$Events = RadioGroupPrimitive.ItemEvents;

  let className: $$Props["class"] = undefined;
  export let value: $$Props["value"];
  export { className as class };
</script>

<RadioGroupPrimitive.Item
  {value}
  class={cn(
    "border-primary text-primary focus-visible:ring-ring aspect-square size-4 rounded-full border shadow focus:outline-none focus-visible:ring-1 disabled:cursor-not-allowed disabled:opacity-50",
    className,
  )}
  {...$$restProps}
  on:click
>
  <div class="flex-center">
    <RadioGroupPrimitive.ItemIndicator>
      <Check class="fill-primary size-3.5" />
    </RadioGroupPrimitive.ItemIndicator>
  </div>
</RadioGroupPrimitive.Item>
