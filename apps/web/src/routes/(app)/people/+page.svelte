<script lang="ts">
  import { inview } from "svelte-inview";
  import PlusCircled from "svelte-radix/PlusCircled.svelte";
  import MixerHorizontal from "svelte-radix/MixerHorizontal.svelte";
  import { useGetPeopleQuery } from "$lib/api/people";
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
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
    <Card.Root>
      <Card.Header class="pb-8">
        <Card.Title class="text-xl font-medium tracking-tight"
          >&ldquo;Imagine all the <span class="text-3xl font-semibold">People</span>...</Card.Title
        >
        <Card.Description>
          <span>...living life in peace.&rdquo;</span>
          <span class="italic">&mdash; John Lennon (1971), &ldquo;Imagine&rdquo;</span>
        </Card.Description>
      </Card.Header>
      <Card.Content>
        <div class="mb-4 flex flex-row items-center space-x-2">
          <Input type="search" placeholder="Search for people" class="h-8 w-[150px] lg:w-[250px]" />
          <Button variant="outline" size="sm" class="h-8 border-dashed">
            <PlusCircled class="mr-2 h-4 w-4" />
            <span>Person</span>
          </Button>
          <div class="flex-1" />
          <Button variant="outline" size="sm" class="h-8">
            <MixerHorizontal class="mr-2 h-4 w-4" />
            <span>View</span>
          </Button>
        </div>
        <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
          <!-- TODO: add ui for empty data -->
          {#each $people.data?.pages || [] as page (page.page)}
            {#if Array.isArray(page.data)}
              {#each page.data as person (person.id)}
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
              class="col-span-full mt-8 flex items-center justify-center"
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
      </Card.Content>
    </Card.Root>
  </div>
</div>
