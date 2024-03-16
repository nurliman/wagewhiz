<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";

  type Activity = {
    title: string;
    description: string;
    date: string;
  };

  const activities: Activity[] = [
    {
      title: "New Product Launch",
      description: "Check out our latest product release!",
      date: "2 days ago",
    },
    {
      title: "Company Town Hall",
      description: "Join the quarterly town hall meeting.",
      date: "1 week ago",
    },
    {
      title: "Hackathon Challenge",
      description: "Participate in our annual hackathon!",
      date: "2 weeks ago",
    },
  ];
</script>

<Card.Root>
  <Card.Header class="bg-primary rounded-t-xl">
    <Card.Title class="text-primary-foreground text-xl font-semibold">Activities</Card.Title>
  </Card.Header>

  <Card.Content class="mt-6 space-y-6">
    {#each activities as activity (activity.title + activity.date)}
      <div class="bg-muted rounded-md p-4">
        <div class="flex items-center space-x-4">
          <img
            src="https://via.placeholder.com/48"
            alt="Profile Picture"
            class="aspect-square h-12 w-12 rounded-full object-cover"
            width="48"
            height="48"
            loading="lazy"
          />
          <div>
            <h4 class="text-foreground font-semibold">{activity.title}</h4>
            <p class="text-muted-foreground">{activity.description}</p>
            <Button variant="link" size="sm" class="px-0 text-sm" href="#">View details</Button>
          </div>
        </div>
        <p class="text-muted-foreground/70 mt-2 text-sm">{activity.date}</p>
      </div>
    {/each}
  </Card.Content>
</Card.Root>
