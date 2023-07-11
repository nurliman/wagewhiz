<script lang="ts">
  import "@fontsource-variable/nunito";
  import "@skeletonlabs/skeleton/styles/skeleton.css";
  import "../app.css";
  import "../theme.css";
  import Navbar from "$lib/components/Navbar.svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import Footer from "$lib/components/Footer.svelte";
  import { computePosition, autoUpdate, offset, shift, flip, arrow } from "@floating-ui/dom";
  import { storePopup } from "@skeletonlabs/skeleton";
  import type { LayoutData } from "./$types.ts";

  storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

  export let data: LayoutData;
</script>

<Navbar />

<div class="flex h-full relative pt-16">
  <Sidebar />
  <main class="flex flex-col h-full w-full lg:ml-64">
    <slot />
    <div class="flex-1" />
    <Footer currentYear={data.currentDate?.getFullYear?.()} />
  </main>
</div>
