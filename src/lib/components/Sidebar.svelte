<script>
  import { fade } from "svelte/transition";
  import { page } from "$app/stores";
  import Sidebar from "flowbite-svelte/Sidebar.svelte";
  import SidebarGroup from "flowbite-svelte/SidebarGroup.svelte";
  import SidebarItem from "flowbite-svelte/SidebarItem.svelte";
  import SidebarWrapper from "flowbite-svelte/SidebarWrapper.svelte";
  import SidebarDropdownWrapper from "flowbite-svelte/SidebarDropdownWrapper.svelte";
  import SidebarDropdownItem from "flowbite-svelte/SidebarDropdownItem.svelte";
  import sidebarStore, { closeSidebar } from "$lib/stores/sidebar.ts";

  $: activeUrl = $page.url.pathname;
</script>

{#if $sidebarStore.show}
  <button class="fixed backdrop" transition:fade={{ duration: 200 }} on:click={closeSidebar} />
{/if}

<div class="sidebar" class:show={$sidebarStore.show}>
  <Sidebar class="border-r border-gray-200 dark:border-gray-700 h-full">
    <SidebarWrapper class="bg-white h-full">
      <SidebarGroup>
        <SidebarItem label="Dashboard" active={activeUrl === "/dashboard"} href="/dashboard" />
        <SidebarItem
          label="People"
          active={activeUrl?.startsWith?.("/people")}
          href="/people"
        />
        <SidebarDropdownWrapper label="E-commerce">
          <SidebarDropdownItem
            label="Products"
            href="/components/products"
            active={activeUrl === "/components/products"}
          />
          <SidebarDropdownItem
            label="Sidebar"
            href="/docs/components/sidebar"
            active={activeUrl === "/docs/components/sidebar"}
          />
        </SidebarDropdownWrapper>
        <SidebarDropdownWrapper label="Items">
          <SidebarDropdownItem
            label="Item 1"
            href="/components/item1"
            active={activeUrl === "/components/item"}
          />
          <SidebarDropdownItem
            label="Item 2"
            href="/components/item2"
            active={activeUrl === "/components/billing"}
          />
        </SidebarDropdownWrapper>
      </SidebarGroup>
    </SidebarWrapper>
  </Sidebar>
</div>

<style lang="scss">
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
</style>
