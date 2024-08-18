<script lang="ts">
  import { page } from "$app/stores";
  import { useGetPersonQuery } from "$lib/api/people";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import * as Breadcrumb from "$lib/components/ui/breadcrumb";
  import Profile from "./Profile.svelte";
  import Activities from "./Activities.svelte";
  import Resources from "./Resources.svelte";

  const personID = $page.params.personID;
  const person = useGetPersonQuery(personID);
</script>

<div class="container">
  <Breadcrumb.Root class="mb-8 px-4">
    <Breadcrumb.List>
      <Breadcrumb.Item>
        <Breadcrumb.Link href="/">Home</Breadcrumb.Link>
      </Breadcrumb.Item>
      <Breadcrumb.Separator />
      <Breadcrumb.Item>
        <Breadcrumb.Link href="/people">People</Breadcrumb.Link>
      </Breadcrumb.Item>
      <Breadcrumb.Separator />
      <Breadcrumb.Item>
        <Breadcrumb.Page>
          {#if $person.status === "pending"}
            <Skeleton class="h-6 w-24" />
          {:else}
            <span>{$person.data?.name || personID}</span>
          {/if}
        </Breadcrumb.Page>
      </Breadcrumb.Item>
    </Breadcrumb.List>
  </Breadcrumb.Root>

  <div class="grid grid-cols-3 gap-8">
    {#if $person.data}
      <Profile person={$person.data} />
    {/if}

    <div class="col-span-2">
      <Activities />

      <div class="mb-8"></div>

      <Resources />
    </div>
  </div>
</div>
