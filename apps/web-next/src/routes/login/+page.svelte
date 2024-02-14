<script lang="ts">
  import type { FormOptions } from "formsnap";
  import type { SuperValidated } from "sveltekit-superforms";
  import type { UserWithCredential } from "$lib/types";
  import { onMount } from "svelte";
  import { isAxiosError } from "axios";
  import { createMutation } from "@tanstack/svelte-query";
  import { toast } from "svelte-sonner";
  import { isLoggedIn } from "$lib/stores/auth";
  import { login } from "$lib/api/auth";
  import { loginInputSchema, type LoginInput } from "$lib/schemas/auth";
  import { goto } from "$app/navigation";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import * as Form from "$lib/components/ui/form";
  import SpinnerPage from "$lib/components/SpinnerPage.svelte";

  const loginMutation = createMutation<UserWithCredential, Error, LoginInput>({
    mutationFn: login,
    onSuccess: async () => {
      $isLoggedIn = true;

      const query = new URLSearchParams(window.location.search);
      const redirectUrl = query?.get?.("redirect") || "/dashboard";
      await goto(redirectUrl);
    },
    onError: (err) => {
      if (isAxiosError(err)) {
        toast.error(err.response?.data?.message || "An error occurred, please try again.");
      }

      console.error(err);
    },
  });

  export let form: SuperValidated<typeof loginInputSchema>;

  const options: FormOptions<typeof loginInputSchema> = {
    SPA: true,
    onUpdate({ form }) {
      if (form.valid) {
        $loginMutation.mutate(form.data);
      }
    },
  };

  let isLoggedInState: boolean | undefined;

  onMount(() => {
    // check if user is already logged in
    if ($isLoggedIn) {
      return goto("/dashboard");
    }

    isLoggedInState = $isLoggedIn ?? false;
  });
</script>

{#if typeof isLoggedInState === "boolean" && !isLoggedInState}
  <Card.Root>
    <Card.Header class="space-y-1">
      <Card.Title class="text-2xl">Login</Card.Title>
      <Card.Description>Enter your email/username and password to login.</Card.Description>
    </Card.Header>
    <Card.Content class="flex flex-col space-y-4">
      <Form.Root
        {form}
        {options}
        schema={loginInputSchema}
        class="flex flex-col space-y-4"
        let:config
      >
        <Form.Field {config} name="username">
          <Form.Item>
            <Form.Label>Email/Username</Form.Label>
            <Form.Input placeholder="Enter your email or username" />
            <Form.Validation />
          </Form.Item>
        </Form.Field>
        <Form.Field {config} name="password">
          <Form.Item>
            <Form.Label>Password</Form.Label>
            <Form.Input type="password" placeholder="Enter your password" />
            <Form.Validation />
          </Form.Item>
        </Form.Field>
        <div>
          <Form.Button class="w-full">Login</Form.Button>
        </div>
      </Form.Root>
      <div class="relative">
        <div class="absolute inset-0 flex items-center">
          <span class="w-full border-t"></span>
        </div>
        <div class="relative flex justify-center text-xs uppercase">
          <span class="bg-card px-2 text-muted-foreground"> Or continue with </span>
        </div>
      </div>
      <div class="flex flex-row space-x-6">
        <Button class="flex-1" type="button" variant="outline">
          <!-- TODO: Add GitHub icon -->
          GitHub
        </Button>
        <Button class="flex-1" type="button" variant="outline">
          <!-- TODO: Add Google icon -->
          Google
        </Button>
      </div>
    </Card.Content>
  </Card.Root>
{:else}
  <SpinnerPage />
{/if}

<style lang="postcss">
  :global(main) {
    @apply p-4 md:p-10 mx-auto max-w-xl w-full;
  }
</style>
