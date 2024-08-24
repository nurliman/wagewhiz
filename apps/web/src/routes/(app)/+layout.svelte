<script lang="ts">
  import { useGetMeQuery } from "$lib/api/auth";
  import AppHeader from "$lib/components/AppHeader.svelte";
  import AppNav from "$lib/components/AppNav.svelte";
  import SpinnerPage from "$lib/components/SpinnerPage.svelte";
  import Footer from "$lib/components/Footer.svelte";

  const me = useGetMeQuery();
</script>

{#if $me.status === "success"}
  <div class="flex size-full flex-1 flex-col">
    <AppHeader />

    <AppNav />

    <main class="flex-1 py-6 lg:py-8">
      <slot />
    </main>

    <Footer />
  </div>
{:else if $me.status === "pending"}
  <SpinnerPage />
{:else}
  <span>Error: {$me?.error?.message || "Unknown error"}</span>
{/if}
