<script lang="ts">
  import * as Card from "$lib/components/ui/card";
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import { Input } from "$lib/components/ui/input";
</script>

<Card.Root>
  <Card.Header class="space-y-1">
    <Card.Title class="text-2xl">Login</Card.Title>
    <Card.Description>Enter your email/username and password to login.</Card.Description>
  </Card.Header>
  <Card.Content class="grid gap-4">
    <div class="grid gap-2">
      <Label for="email">Email/Username</Label>
      <Input id="email" type="email" placeholder="Enter your email or username" />
    </div>
    <div class="grid gap-2">
      <Label for="password">Password</Label>
      <Input id="password" type="password" placeholder="Enter your password" />
    </div>
    <div>
      <Button type="submit" class="w-full">Login</Button>
    </div>
    <div class="relative">
      <div class="absolute inset-0 flex items-center">
        <span class="w-full border-t"></span>
      </div>
      <div class="relative flex justify-center text-xs uppercase">
        <span class="bg-card px-2 text-muted-foreground"> Or continue with </span>
      </div>
    </div>
    <div class="grid grid-cols-2 gap-6">
      <Button type="button" variant="outline">
        <!-- TODO: Add GitHub icon -->
        GitHub
      </Button>
      <Button type="button" variant="outline">
        <!-- TODO: Add Google icon -->
        Google
      </Button>
    </div>
  </Card.Content>
</Card.Root>

<style>
  :global(main) {
    @apply p-4 md:p-10 mx-auto max-w-xl w-full;
  }
</style>
