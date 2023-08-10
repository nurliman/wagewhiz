<script lang="ts">
  import "@fontsource-variable/nunito";
  import "@skeletonlabs/skeleton/styles/skeleton.css";
  import "../app.postcss";
  import "../theme.postcss";
  import { computePosition, autoUpdate, offset, shift, flip, arrow } from "@floating-ui/dom";
  import { storePopup } from "@skeletonlabs/skeleton";

  storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });
</script>

<slot />
