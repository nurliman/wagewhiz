<script lang="ts">
  import { useGetMeQuery } from "$lib/api/auth";
  import SpinnerPage from "$lib/components/SpinnerPage.svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";

  const me = useGetMeQuery();
</script>

{#if $me.status === "success"}
  <div class="flex h-full w-full flex-1 flex-row">
    <Sidebar />

    <main class="flex-1">
      <slot />
    </main>
  </div>
{:else if $me.status === "pending"}
  <SpinnerPage />
{:else}
  <span>Error: {$me?.error?.message || "Unknown error"}</span>
{/if}
