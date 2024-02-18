<script lang="ts">
  import { useGetMeQuery } from "$lib/api/auth";
  import AppHeader from "$lib/components/AppHeader.svelte";
  import AppNav from "$lib/components/AppNav.svelte";
  import SpinnerPage from "$lib/components/SpinnerPage.svelte";

  const me = useGetMeQuery();
</script>

{#if $me.status === "success"}
  <div class="flex h-full w-full flex-1 flex-col">
    <AppHeader />

    <AppNav />

    <main class="flex-1">
      <slot />
    </main>
  </div>
{:else if $me.status === "pending"}
  <SpinnerPage />
{:else}
  <span>Error: {$me?.error?.message || "Unknown error"}</span>
{/if}
