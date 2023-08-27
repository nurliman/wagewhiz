<script lang="ts">
  import Navbar from "$lib/components/Navbar.svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import Footer from "$lib/components/Footer.svelte";
  import SpinnerPage from "$lib/components/SpinnerPage.svelte";
  import { getMeQuery } from "$lib/apis/authApi.ts";
  import type { LayoutData } from "./$types.ts";

  const me = getMeQuery();

  export let data: LayoutData;
</script>

{#if $me.status === "loading"}
  <SpinnerPage />
{:else if $me.status === "error"}
  <span>Error: {$me.error.message}</span>
{:else}
  <Navbar />
  <div class="flex h-full relative pt-16">
    <Sidebar />
    <main class="flex flex-col h-full w-full lg:ml-64">
      <slot />
      <div class="flex-1" />
      <Footer currentYear={data.currentDate?.getFullYear?.()} />
    </main>
  </div>
{/if}
