<script lang="ts">
  import { faker } from "@faker-js/faker";
  import { nanoid } from "$lib/utils/nanoid.ts";
  import { Avatar, popup } from "@skeletonlabs/skeleton";
  import EmploymentStatusBadge from "$lib/components/EmploymentStatusBadge.svelte";
  import CaretDownIcon from "~icons/ph/caret-down";
  import type { PopupSettings } from "@skeletonlabs/skeleton";
  import type { Employee } from "$lib/types.ts";

  export let person: Employee;

  const menuPopupId = nanoid();
  const menuPopup: PopupSettings = {
    event: "click",
    target: menuPopupId,
    placement: "bottom",
  };
</script>

<div class=" card container !p-4 !pb-6">
  <div class="flex flex-col items-center space-y-4">
    <div class="flex items-start w-full">
      <input class="checkbox checked:!bg-secondary-500" type="checkbox" />
      <div class="flex-1" />
      <div class="btn-group variant-soft-surface dark:bg-primary-500/20">
        <a class="!py-1.5 !px-2 !text-xs" href={`/people/${person.id}/edit`}>Edit</a>
        <button class="!py-1.5 !px-2 !text-xs" use:popup={menuPopup}>
          <CaretDownIcon />
        </button>
      </div>
    </div>
    <Avatar
      src={faker.image.urlLoremFlickr({ category: "abstract" })}
      border="border-4 border-surface-300-600-token hover:!border-primary-500"
      width="w-20"
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
  <div class="card p-2 shadow-xl" data-popup={menuPopupId}>
    <nav class="list-nav text-sm text-gray-700 dark:text-white">
      <ul>
        <li><a href="/">Menu 1</a></li>
        <li><a href="/">Menu 2</a></li>
        <li><a href="/">Menu 3</a></li>
        <li><a href="/">Menu 4</a></li>
      </ul>
    </nav>
  </div>
</div>
