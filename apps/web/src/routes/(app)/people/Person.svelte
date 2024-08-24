<script lang="ts">
  import type { Person } from "$lib/types";
  import { Button } from "$lib/components/ui/button";
  import { Badge } from "$lib/components/ui/badge";
  import FluentSend16Regular from "virtual:icons/fluent/send-16-regular";
  import FluentMoreVertical20Regular from "virtual:icons/fluent/more-vertical-20-regular";
  import * as Avatar from "$lib/components/ui/avatar";
  import * as Card from "$lib/components/ui/card";
  import initials from "initials";

  export let person: Person;

  const safeName = person.name || "Guest";
  const personInitials = initials(safeName);
  const personHref = `/people/${person.id}`;
</script>

<Card.Root>
  <Card.Content>
    <div class="mb-6"></div>
    <div class="flex flex-col items-center">
      <a href={personHref} class="mb-4">
        <Avatar.Root class="size-20">
          <Avatar.Image src={person.avatar_url} alt={safeName} loading="lazy" />
          <Avatar.Fallback class="text-2xl">{personInitials}</Avatar.Fallback>
        </Avatar.Root>
      </a>
      <div class="text-center">
        {#if person.status}
          <Badge variant="outline" class="mb-1.5">
            {person.status}
          </Badge>
        {/if}

        <a href={personHref}>
          <div class="text-lg font-semibold hover:underline">{safeName}</div>
          {#if person.role}
            <div class="text-muted-foreground text-sm">{person.role}</div>
          {/if}
        </a>
      </div>
    </div>
  </Card.Content>
  <Card.Footer class="flex space-x-2">
    <Button variant="secondary" size="sm" class="h-8 flex-1">
      <FluentSend16Regular class="mr-2 size-4" />
      <span>Send Message</span>
    </Button>
    <Button variant="secondary" size="icon" class="h-8">
      <FluentMoreVertical20Regular class="size-5" />
    </Button>
  </Card.Footer>
</Card.Root>
