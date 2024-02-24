<script lang="ts">
  import PlusCircled from "svelte-radix/PlusCircled.svelte";
  import MixerHorizontal from "svelte-radix/MixerHorizontal.svelte";
  import { people } from "$lib/data/people";
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import Person from "./Person.svelte";
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
          {#each people as person (person.id)}
            <Person {person} />
          {/each}
        </div>
      </Card.Content>
    </Card.Root>
  </div>
</div>
