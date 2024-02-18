<script lang="ts">
  import { goto } from "$app/navigation";
  import { toast } from "svelte-sonner";
  import { isAxiosError } from "axios";
  import { createMutation } from "@tanstack/svelte-query";
  import { isLoggedIn } from "$lib/stores/auth";
  import { logout } from "$lib/api/auth";
  import { Button } from "$lib/components/ui/button";
  import * as Avatar from "$lib/components/ui/avatar";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";

  const logoutMutation = createMutation<null, Error>({
    mutationFn: logout,
    onSuccess() {
      isLoggedIn.set(false);
      
      goto("/login");

      toast.success("You have been logged out.");
    },
    onError(err) {
      if (isAxiosError(err)) {
        toast.error(err.response?.data?.message || "An error occurred, please try again.");
      }

      console.error(err);
    },
  });
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger>
    <Button variant="outline" size="icon" class="flex-center h-10 w-10 rounded-full">
      <Avatar.Root class="h-8 w-8">
        <Avatar.Image src="https://github.com/shadcn.png" alt="Avatar" />
        <Avatar.Fallback>CN</Avatar.Fallback>
      </Avatar.Root>
    </Button>
  </DropdownMenu.Trigger>
  <DropdownMenu.Content>
    <DropdownMenu.Group>
      <DropdownMenu.Label>
        <div class="flex flex-col space-y-0.5 font-normal">
          <div>Nurliman Diara</div>
          <div class="text-sm text-gray-500">example@email.com</div>
        </div>
      </DropdownMenu.Label>
      <DropdownMenu.Separator />
      <DropdownMenu.Item>Profile</DropdownMenu.Item>
      <DropdownMenu.Item>Billing</DropdownMenu.Item>
      <DropdownMenu.Item>Team</DropdownMenu.Item>
      <DropdownMenu.Item>Subscription</DropdownMenu.Item>
      <DropdownMenu.Separator />
      <DropdownMenu.Item on:click={() => $logoutMutation.mutate()}>Logout</DropdownMenu.Item>
    </DropdownMenu.Group>
  </DropdownMenu.Content>
</DropdownMenu.Root>
