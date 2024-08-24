<script lang="ts">
  import { Avatar as AvatarPrimitive } from "bits-ui";
  import { cn } from "$lib/utils/shadcn.ts";

  type $$Props = AvatarPrimitive.FallbackProps;

  let className: $$Props["class"] = undefined;
  export { className as class };
</script>

<AvatarPrimitive.Fallback
  class={cn("bg-muted flex-center size-full rounded-full", className)}
  {...$$restProps}
>
  <slot />
</AvatarPrimitive.Fallback>
