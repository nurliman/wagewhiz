<script lang="ts">
  import { useGetMeQuery } from "$lib/api/auth";
  import SpinnerPage from "$lib/components/SpinnerPage.svelte";

  const me = useGetMeQuery();
</script>

{#if $me.status === "success"}
  <div class="flex h-full w-full flex-1 flex-row">
    <aside class="flex flex-col">
      <nav class="flex flex-col">
        <ul class="flex flex-col">
          <li><a href="/dashboard">Dashboard</a></li>
          <li><a href="/people">People</a></li>
          <li><a href="/attendance">Time & Attendance</a></li>
          <li><a href="/payroll">Payroll</a></li>
          <li><a href="/recruitment">Recruitment</a></li>
          <li><a href="/insights">Insights</a></li>
          <li><a href="/settings">Settings</a></li>
        </ul>
      </nav>
    </aside>

    <main class="flex-1">
      <slot />
    </main>
  </div>
{:else if $me.status === "pending"}
  <SpinnerPage />
{:else}
  <span>Error: {$me?.error?.message || "Unknown error"}</span>
{/if}
