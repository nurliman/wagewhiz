<script lang="ts">
  import "@fontsource-variable/nunito";
  import "@skeletonlabs/skeleton/styles/skeleton.css";
  import "../app.postcss";
  import "../theme.postcss";
  import { computePosition, autoUpdate, offset, shift, flip, arrow } from "@floating-ui/dom";
  import { storePopup } from "@skeletonlabs/skeleton";
  import { SvelteToast } from "svelte-toast";
  import { QueryClientProvider } from "@tanstack/svelte-query";
  import { theQueryClient } from "$lib/libs/theQueryClient.ts";

  storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });
</script>

<QueryClientProvider client={theQueryClient}>
  <slot />
</QueryClientProvider>

<SvelteToast options={{ reversed: true, intro: { y: 192 } }} />
