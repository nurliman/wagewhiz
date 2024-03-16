<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";

  type Resource = {
    title: string;
    description: string;
    link: string;
  };

  const resources: Resource[] = [
    {
      title: "Learning & Development",
      description: "Access a wide range of training courses and career development resources.",
      link: "#",
    },
    {
      title: "Health & Wellness",
      description: "Explore our wellness programs and employee assistance services.",
      link: "#",
    },
    {
      title: "Employee Discounts",
      description: "Enjoy exclusive discounts and perks from our partners.",
      link: "#",
    },
  ];
</script>

<Card.Root>
  <Card.Header class="rounded-t-xl">
    <Card.Title class="text-xl font-semibold">Employee Resources</Card.Title>
  </Card.Header>

  <Card.Content class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
    {#each resources as resource (resource.title)}
      <div class="bg-muted rounded-lg p-4">
        <h4 class="text-foreground font-semibold">{resource.title}</h4>
        <p class="text-muted-foreground mt-2">{resource.description}</p>
        <Button variant="link" size="sm" class="mt-2 px-0 text-sm" href="#">Learn more</Button>
      </div>
    {/each}
  </Card.Content>
</Card.Root>
