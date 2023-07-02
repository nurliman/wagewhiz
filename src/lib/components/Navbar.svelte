<script>
  import Navbar from "flowbite-svelte/Navbar.svelte";
  import NavBrand from "flowbite-svelte/NavBrand.svelte";
  import NavHamburger from "flowbite-svelte/NavHamburger.svelte";
  import Avatar from "flowbite-svelte/Avatar.svelte";
  import DarkMode from "flowbite-svelte/DarkMode.svelte";
  import Dropdown from "flowbite-svelte/Dropdown.svelte";
  import DropdownItem from "flowbite-svelte/DropdownItem.svelte";
  import DropdownHeader from "flowbite-svelte/DropdownHeader.svelte";
  import DropdownDivider from "flowbite-svelte/DropdownDivider.svelte";
  import { toggleSidebar } from "$lib/stores/sidebar.ts";
</script>

<Navbar
  color="default"
  navClass="px-3 py-3 lg:px-5 lg:pl-3 fixed w-full z-30 top-0 left-0 border-b"
  classNavDiv="max-w-none"
>
  <div class="flex items-center">
    <NavHamburger on:click={toggleSidebar} class="ml-0.5 md:flex lg:hidden" />
    <NavBrand href="/" class="flex ml-2 md:mr-24">
      <span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">
        [ WageWhiz ]
      </span>
    </NavBrand>
  </div>
  <div class="flex items-center justify-end ml-auto">
    <DarkMode />
    <Avatar id="avatar-menu" src="/images/profile-picture-3.webp" class="ml-3" />
  </div>
  <Dropdown placement="bottom" triggeredBy="#avatar-menu">
    <DropdownHeader>
      <span class="block text-sm"> Bonnie Green </span>
      <span class="block truncate text-sm font-medium"> name@flowbite.com </span>
    </DropdownHeader>
    <DropdownItem>Dashboard</DropdownItem>
    <DropdownItem>Settings</DropdownItem>
    <DropdownItem>Earnings</DropdownItem>
    <DropdownDivider />
    <DropdownItem>Sign out</DropdownItem>
  </Dropdown>
</Navbar>
