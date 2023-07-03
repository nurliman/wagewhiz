<script>
  import Breadcrumb from "flowbite-svelte/Breadcrumb.svelte";
  import BreadcrumbItem from "flowbite-svelte/BreadcrumbItem.svelte";
</script>

<div class="grid grid-cols-1 px-4 pt-4 gap-4 dark:bg-gray-900">
  <div class="xl:mb-2">
    <Breadcrumb class="mb-5">
      <BreadcrumbItem href="/" home>Home</BreadcrumbItem>
      <BreadcrumbItem href="/people">People</BreadcrumbItem>
      <BreadcrumbItem>Edit Person Information</BreadcrumbItem>
    </Breadcrumb>
    <h1 class="text-xl font-semibold text-gray-900 sm:text-2xl dark:text-white">
      Edit Person Information
    </h1>
  </div>
</div>
