<script lang="ts">
  import "@fontsource-variable/inter";
  import "../app.css";
  import Navbar from "$lib/components/Navbar.svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import Footer from "$lib/components/Footer.svelte";
  import type { LayoutData } from "./$types.ts";

  export let data: LayoutData;
</script>

<Navbar />

<div class="flex h-full relative pt-16 text-gray-900 dark:text-gray-100">
  <Sidebar />
  <main class="flex flex-col h-full w-full lg:ml-64">
    <slot />
    <div class="flex-1" />
    <Footer currentYear={data.currentDate?.getFullYear?.()} />
  </main>
</div>
