<script lang="ts">
  import * as Card from "$lib/components/ui/card";
  import { Skeleton } from "$lib/components/ui/skeleton";
</script>

<Card.Root class="flex h-[280px] w-full flex-col">
  <Card.Content class="flex-1">
    <div class="mb-6"></div>
    <div class="flex flex-col items-center">
      <Skeleton class="mb-4 size-20 rounded-full" />
      <div class="flex flex-col items-center">
        <Skeleton class="mb-2 h-5 w-24" />
        <Skeleton class="mb-1 h-6 w-40" />
        <Skeleton class="h-4 w-32" />
      </div>
    </div>
  </Card.Content>
  <Card.Footer>
    <Skeleton class="h-8  w-full" />
  </Card.Footer>
</Card.Root>
