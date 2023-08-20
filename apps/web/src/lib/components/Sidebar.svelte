<script>
  import { fade } from "svelte/transition";
  import { page } from "$app/stores";
  import { closeSidebar, sidebarStore } from "$lib/stores/sidebar.ts";

  $: activeUrl = $page.url.pathname;
</script>

{#if $sidebarStore.show}
  <button class="fixed backdrop" transition:fade={{ duration: 200 }} on:click={closeSidebar} />
{/if}

<div class="sidebar" class:show={$sidebarStore.show}>
  <aside class="w-64 border-r border-gray-200 dark:border-surface-500 h-full" aria-label="Sidebar">
    <div class="overflow-y-auto py-4 px-3 rounded bg-white dark:bg-surface-800 h-full">
      <nav class="list-nav">
        <ul>
          <li><a href="/dashboard" class:active={activeUrl === "/dashboard"}>Dashboard</a></li>
          <li><a href="/people" class:active={activeUrl?.startsWith?.("/people")}>People</a></li>
        </ul>
      </nav>
    </div>
  </aside>
</div>

<style lang="postcss">
  .sidebar {
    position: fixed;
    height: 100%;
    z-index: 20;
    transform: translate3d(-100%, 0, 0);

    @apply transition-all;

    &.show {
      transform: translate3d(0, 0, 0);
    }

    @media (min-width: 1024px) {
      transform: translate3d(0, 0, 0);
    }
  }

  .backdrop {
    position: fixed;
    height: 100%;
    width: 100%;
    z-index: 10;

    @apply backdrop-blur-sm;
    @apply bg-gray-900/20;
    @apply dark:bg-gray-900/50;

    @media (min-width: 1024px) {
      display: none;
    }
  }

  .active {
    @apply bg-gray-200 dark:bg-gray-700;
    @apply hover:bg-gray-100 dark:hover:bg-gray-700;
  }
</style>
