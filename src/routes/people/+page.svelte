<script lang="ts">
  import { faker } from "@faker-js/faker";
  import Avatar from "flowbite-svelte/Avatar.svelte";
  import Badge from "flowbite-svelte/Badge.svelte";
  import Breadcrumb from "flowbite-svelte/Breadcrumb.svelte";
  import BreadcrumbItem from "flowbite-svelte/BreadcrumbItem.svelte";
  import Button from "flowbite-svelte/Button.svelte";
  import ButtonGroup from "flowbite-svelte/ButtonGroup.svelte";
  import Card from "flowbite-svelte/Card.svelte";
  import Dropdown from "flowbite-svelte/Dropdown.svelte";
  import DropdownItem from "flowbite-svelte/DropdownItem.svelte";
  import GradientButton from "flowbite-svelte/GradientButton.svelte";
  import Checkbox from "flowbite-svelte/Checkbox.svelte";
  import Search from "flowbite-svelte/Search.svelte";
  import EmploymentStatusBadge from "$lib/components/EmploymentStatusBadge.svelte";
  import ExportIcon from "~icons/ph/export";
  import FunnelIcon from "~icons/ph/funnel";
  import BabyIcon from "~icons/ph/baby-fill";
  import CaretDownIcon from "~icons/ph/caret-down";
  import type { PageData } from "./$types.ts";

  export let data: PageData;
</script>

<div class="p-4 block sm:flex items-center justify-between">
  <div class="w-full mb-1">
    <div class="mb-4">
      <Breadcrumb class="mb-5">
        <BreadcrumbItem href="/" home>Home</BreadcrumbItem>
        <BreadcrumbItem>People</BreadcrumbItem>
      </Breadcrumb>
      <div class="flex items-center">
        <h1 class="text-xl font-semibold text-gray-900 sm:text-2xl md:text-3xl dark:text-white">
          People
        </h1>
        <div class="w-2" />
        <Badge border color="blue" rounded>{data.peopleCount}</Badge>
      </div>
    </div>
    <div class="items-center justify-between block sm:flex">
      <div class="flex items-center space-x-2 sm:space-x-3 mb-4 sm:mb-0">
        <form>
          <Search
            class="min-h-[38px] w-full sm:w-64 xl:w-96"
            size="md"
            placeholder="Search For People"
          />
        </form>
        <div>
          <Button class="!p-2" size="lg" color="alternative">
            <FunnelIcon />
          </Button>
        </div>
      </div>
      <div class="flex items-center ml-auto space-x-2 sm:space-x-3">
        <GradientButton size="sm" color="blue" href="/people/new">
          <BabyIcon class="mr-2" height={20} width={20} />
          <span>New Person</span>
        </GradientButton>
        <Button size="sm" color="alternative">
          <ExportIcon class="mr-2" />
          <span>Export</span>
        </Button>
      </div>
    </div>
  </div>
</div>

<div class="px-4 grid gap-4 grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
  {#each data.people as person (person.id)}
    <Card class="max-w-none !p-4 !pb-6">
      <div class="flex flex-col items-center space-y-4">
        <div class="flex items-start w-full">
          <Checkbox />
          <div class="flex-1" />
          <ButtonGroup class="space-x-px shadow-none" size="xs">
            <Button class="focus:ring-1" size="xs" href={`/people/${person.id}/edit`}>Edit</Button>
            <Button class="focus:ring-1" size="xs" id={`person-${person.id}-menu`}>
              <CaretDownIcon />
            </Button>
          </ButtonGroup>
        </div>
        <Avatar
          class="text-3xl"
          border
          size="lg"
          src={faker.image.urlLoremFlickr({ category: "abstract" })}
        />
        <div class="text-center">
          <EmploymentStatusBadge status={person.status} class="mb-1.5" />
          <div class="text-lg font-semibold text-gray-900 dark:text-white">
            {person.name}
          </div>
          <div class="text-sm font-normal text-gray-500 dark:text-gray-400">
            {person.position}
          </div>
          <div />
        </div>
      </div>
      <Dropdown placement="bottom-end" triggeredBy={`#person-${person.id}-menu`}>
        <DropdownItem>Menu 1</DropdownItem>
        <DropdownItem>Menu 2</DropdownItem>
        <DropdownItem>Menu 3</DropdownItem>
        <DropdownItem>Menu 4</DropdownItem>
      </Dropdown>
    </Card>
  {/each}
</div>
