<script lang="ts">
  import { inview } from "svelte-inview";
  import FluentFilter20Regular from "virtual:icons/fluent/filter-20-regular";
  import FluentArrowSort20Regular from "virtual:icons/fluent/arrow-sort-20-regular";
  import FluentPeopleAdd20Regular from "virtual:icons/fluent/people-add-20-regular";
  import FluentPersonAdd20Regular from "virtual:icons/fluent/person-add-20-regular";
  import { useGetPeopleQuery } from "$lib/api";
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import * as Breadcrumb from "$lib/components/ui/breadcrumb";
  import Person from "./Person.svelte";
  import PersonSkeleton from "./PersonSkeleton.svelte";

  const people = useGetPeopleQuery();

  const fetchNextPage = () => {
    if (!$people.isFetched) return;
    if (!$people.hasNextPage) return;
    if ($people.isFetching) return;
    if ($people.isFetchingNextPage) return;

    $people.fetchNextPage();
  };
</script>

<div>
  <div class="container">
    <Breadcrumb.Root class="mb-8 px-4">
      <Breadcrumb.List>
        <Breadcrumb.Item>
          <Breadcrumb.Link href="/">Home</Breadcrumb.Link>
        </Breadcrumb.Item>
        <Breadcrumb.Separator />
        <Breadcrumb.Item>
          <Breadcrumb.Page>People</Breadcrumb.Page>
        </Breadcrumb.Item>
      </Breadcrumb.List>
    </Breadcrumb.Root>

    <Card.Root>
      <Card.Header>
        <Card.Title class="text-xl font-medium tracking-tight"
          >&ldquo;Imagine all the <span class="text-3xl font-semibold">People</span>...</Card.Title
        >
        <Card.Description>
          <span>...living life in peace.&rdquo;</span>
          <span class="italic">&mdash; John Lennon (1971), &ldquo;Imagine&rdquo;</span>
        </Card.Description>
      </Card.Header>
      <Card.Content class="flex flex-row items-center space-x-2">
        <Input type="search" placeholder="Search for people" class="w-[150px] lg:w-[250px]" />

        <Button variant="outline" class="">
          <FluentFilter20Regular class="mr-2 size-4" />
          <span>Filter</span>
        </Button>

        <Button variant="outline" class="">
          <FluentArrowSort20Regular class="mr-2 size-4" />
          <span>Sort</span>
        </Button>

        <div class="flex-1"></div>

        <Button variant="outline" class="">
          <FluentPeopleAdd20Regular class="mr-2 size-4" />
          <span>Import People</span>
        </Button>

        <Button variant="default" class="">
          <FluentPersonAdd20Regular class="mr-2 size-4" />
          <span>Add Person</span>
        </Button>
      </Card.Content>
    </Card.Root>

    <div class="mb-4"></div>

    <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
      <!-- TODO: add ui for empty data -->
      {#each $people.data?.pages || [] as page (page.page)}
        {#if Array.isArray(page.items)}
          {#each page.items as person (person.id)}
            <Person {person} />
          {/each}
        {/if}
      {/each}

      {#if $people.isFetching || $people.isFetchingNextPage}
        {#each Array.from({ length: 12 }) as _}
          <PersonSkeleton />
        {/each}
      {:else if $people.isFetched && $people.hasNextPage}
        <div
          use:inview
          class="flex-center col-span-full mt-8"
          on:inview_enter={(e) => e.detail.inView && fetchNextPage()}
        >
          <Button variant="outline" on:click={fetchNextPage}>Load more</Button>
        </div>
      {:else if $people.isFetched}
        <div class="text-muted-foreground col-span-full mt-8 text-center">
          No more people to load.
        </div>
      {/if}
    </div>
  </div>
</div>
