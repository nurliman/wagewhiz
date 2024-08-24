<script lang="ts">
  import type { Person } from "$lib/types";
  import * as Avatar from "$lib/components/ui/avatar";
  import * as Card from "$lib/components/ui/card";
  import initials from "initials";

  type ChronoCardData = {
    title: string;
    subtitle: string;
    date: string;
  };

  const experiences: ChronoCardData[] = [
    {
      title: "Acme Corp",
      subtitle: "Senior Software Engineer",
      date: "Jan 2020 - Present",
    },
    {
      title: "Globex Inc.",
      subtitle: "Software Engineer",
      date: "Jun 2018 - Dec 2019",
    },
  ];

  const education: ChronoCardData[] = [
    {
      title: "MIT",
      subtitle: "Bachelor's in Computer Science",
      date: "2014 - 2018",
    },
  ];

  export let person: Person;

  const safeName = person.name || "Guest";
  const personInitials = initials(safeName);
</script>

{#snippet chronoCard(data: ChronoCardData)}
  <div class="bg-muted mt-4 rounded-md p-4">
    <h4 class="text-foreground font-semibold">{data.title}</h4>
    <p class="text-muted-foreground">{data.subtitle}</p>
    <p class="text-muted-foreground/70 text-sm">{data.date}</p>
  </div>
{/snippet}

<Card.Root class="col-span-1">
  <Card.Header class="bg-primary flex flex-col items-center rounded-t-xl">
    <Avatar.Root class="size-32">
      <Avatar.Image src={person.avatar_url} alt={safeName} loading="lazy" />
      <Avatar.Fallback class="text-4xl">{personInitials}</Avatar.Fallback>
    </Avatar.Root>

    <div class="mt-4 text-center">
      <Card.Title class="text-primary-foreground text-xl font-semibold">{safeName}</Card.Title>
      <Card.Description class="text-primary-foreground/80 text-sm">{person.role}</Card.Description>
    </div>
  </Card.Header>

  <Card.Content class="mt-6">
    <div class="space-y-6">
      <div>
        <h3 class="text-lg font-semibold tracking-tight">About</h3>
        <p class="text-card-foreground/60 mt-2">
          I'm a skilled software engineer with expertise in web development and cloud computing. I
          thrive in collaborative environments and enjoy tackling complex challenges.
        </p>
      </div>
      <div>
        <h3 class="text-lg font-semibold tracking-tight">Experience</h3>
        <div class="mt-4 space-y-4">
          {#each experiences as exp (exp.title + exp.date)}
            {@render chronoCard(exp)}
          {/each}
        </div>
      </div>
      <div>
        <h3 class="text-lg font-semibold tracking-tight">Education</h3>
        <div class="mt-4 space-y-4">
          {#each education as edu (edu.title + edu.date)}
            {@render chronoCard(edu)}
          {/each}
        </div>
      </div>
    </div>
  </Card.Content>
</Card.Root>
