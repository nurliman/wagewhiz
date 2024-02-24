<script lang="ts">
  import type { Employee } from "$lib/types";
  import { Button } from "$lib/components/ui/button";
  import { Badge } from "$lib/components/ui/badge";
  import DotsVertical from "svelte-radix/DotsVertical.svelte";
  import PaperPlane from "svelte-radix/PaperPlane.svelte";
  import * as Avatar from "$lib/components/ui/avatar";
  import * as Card from "$lib/components/ui/card";
  import initials from "initials";

  export let person: Employee;

  const personInitials = initials(person.name);
  const personHref = `/people/${person.id}`;
</script>

<Card.Root>
  <Card.Content>
    <div class="mb-6" />
    <div class="flex flex-col items-center space-y-4">
      <a href={personHref}>
        <Avatar.Root class="h-20 w-20">
          <Avatar.Image src="https://github.com/nurliman.png" alt="@nurliman" loading="lazy" />
          <Avatar.Fallback class="text-2xl">{personInitials}</Avatar.Fallback>
        </Avatar.Root>
      </a>
      <div class="text-center">
        <Badge variant="outline" class="mb-1.5">{person.status}</Badge>
        <a href={personHref}>
          <div class="text-lg font-semibold hover:underline">{person.name}</div>
          <div class="text-muted-foreground text-sm">{person.position}</div>
        </a>
      </div>
    </div>
  </Card.Content>
  <Card.Footer class="flex space-x-2">
    <Button variant="secondary" size="sm" class="flex-1 h-8">
      <PaperPlane class="mr-2 h-3.5 w-3.5" /> 
      <span>Send Message</span>
    </Button>
    <Button variant="secondary" size="icon" class="h-8">
      <DotsVertical class="h-4 w-4" />
    </Button>
  </Card.Footer>
</Card.Root>
