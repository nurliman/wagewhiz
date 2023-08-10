<script lang="ts">
  import PersonCard from "./PersonCard.svelte";
  import BabyIcon from "~icons/ph/baby-fill";
  import CaretRightIcon from "~icons/ph/caret-right";
  import ExportIcon from "~icons/ph/export";
  import FunnelIcon from "~icons/ph/funnel";
  import HouseIcon from "~icons/ph/house-fill";
  import MagnifierIcon from "~icons/ph/magnifying-glass";
  import type { PageData } from "./$types.ts";

  export let data: PageData;
</script>

<div class="p-4 block sm:flex items-center justify-between">
  <div class="w-full mb-1">
    <div class="mb-4">
      <ol class="breadcrumb text-sm space-x-1 mb-5 xl:mb-6">
        <li class="crumb">
          <a href="/" class="btn btn-sm variant-soft">
            <HouseIcon height={14} width={14} />
            <span>Home</span>
          </a>
        </li>
        <li class="crumb-separator" aria-hidden><CaretRightIcon height={14} width={14} /></li>
        <li>People</li>
      </ol>

      <div class="flex items-center">
        <h1 class="h2">People</h1>
        <div class="w-2" />
        <span class="badge variant-ghost-secondary">{data.peopleCount}</span>
      </div>
    </div>
    <div class="items-center justify-between block sm:flex">
      <div class="flex items-center space-x-2 sm:space-x-3 mb-4 sm:mb-0">
        <form>
          <div class="input-group input-group-divider grid-cols-[1fr_auto] w-full sm:w-64 xl:w-96">
            <input type="search" placeholder="Search For People" />
            <button type="submit" class="variant-soft">
              <MagnifierIcon />
            </button>
          </div>
        </form>
        <div>
          <button type="button" class="btn-icon variant-soft rounded-lg">
            <FunnelIcon />
          </button>
        </div>
      </div>
      <div class="flex items-center ml-auto space-x-2 sm:space-x-3">
        <a href="/people/new" class="btn bg-gradient-to-br variant-gradient-primary-secondary">
          <BabyIcon height={20} width={20} />
          <span>New Person</span>
        </a>
        <button type="button" class="btn variant-soft">
          <ExportIcon />
          <span>Export</span>
        </button>
      </div>
    </div>
  </div>
</div>

<div class="px-4 grid gap-4 grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
  {#each data.people as person (person.id)}
    <PersonCard {person} />
  {/each}
</div>
