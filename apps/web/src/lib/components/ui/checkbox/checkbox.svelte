<script lang="ts">
  import { Checkbox as CheckboxPrimitive } from "bits-ui";
  import Check from "svelte-radix/Check.svelte";
  import Minus from "svelte-radix/Minus.svelte";
  import { cn } from "$lib/utils/shadcn.ts";

  type $$Props = CheckboxPrimitive.Props;
  type $$Events = CheckboxPrimitive.Events;

  let className: $$Props["class"] = undefined;
  export let checked: $$Props["checked"] = false;
  export { className as class };
</script>

<CheckboxPrimitive.Root
  class={cn(
    "border-primary focus-visible:ring-ring data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground peer box-content size-4 shrink-0 rounded-sm border shadow focus-visible:outline-none focus-visible:ring-1 disabled:cursor-not-allowed disabled:opacity-50 data-[disabled=true]:cursor-not-allowed data-[disabled=true]:opacity-50",
    className,
  )}
  bind:checked
  on:click
  {...$$restProps}
>
  <CheckboxPrimitive.Indicator
    class={cn("flex-center size-4 text-current")}
    let:isChecked
    let:isIndeterminate
  >
    {#if isIndeterminate}
      <Minus class="size-3.5" />
    {:else}
      <Check class={cn("size-3.5", !isChecked && "text-transparent")} />
    {/if}
  </CheckboxPrimitive.Indicator>
</CheckboxPrimitive.Root>
