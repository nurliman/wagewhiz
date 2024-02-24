<script lang="ts">
  import type { UserWithCredential } from "$lib/types";
  import { superForm, defaults } from "sveltekit-superforms";
  import { valibot } from "sveltekit-superforms/adapters";
  import { isAxiosError } from "axios";
  import { createMutation } from "@tanstack/svelte-query";
  import { toast } from "svelte-sonner";
  import { isLoggedIn } from "$lib/stores/auth";
  import { login } from "$lib/api/auth";
  import { loginInputSchema, type LoginInput } from "$lib/schemas/auth";
  import { goto } from "$app/navigation";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import * as Card from "$lib/components/ui/card";
  import * as Form from "$lib/components/ui/form";

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

  const form = superForm(defaults(valibot(loginInputSchema)), {
    SPA: true,
    validators: valibot(loginInputSchema),
    resetForm: false,
    onUpdate({ form }) {
      if (form.valid) {
        $loginMutation.mutate(form.data);
      }
    },
  });

  const { form: formData, enhance } = form;
</script>

<Card.Root>
  <Card.Header class="space-y-1">
    <Card.Title class="text-2xl">Login</Card.Title>
    <Card.Description>Enter your email/username and password to login.</Card.Description>
  </Card.Header>
  <Card.Content class="flex flex-col space-y-4">
    <form class="flex flex-col space-y-4" method="POST" use:enhance>
      <Form.Field {form} name="username">
        <Form.Control let:attrs>
          <Form.Label>Email/Username</Form.Label>
          <Input
            {...attrs}
            bind:value={$formData.username}
            placeholder="Enter your email or username"
          />
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>
      <Form.Field {form} name="password">
        <Form.Control let:attrs>
          <Form.Label>Password</Form.Label>
          <Input
            {...attrs}
            bind:value={$formData.password}
            type="password"
            placeholder="Enter your password"
          />
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>
      <div>
        <Form.Button class="w-full">Login</Form.Button>
      </div>
    </form>
    <div class="relative">
      <div class="absolute inset-0 flex items-center">
        <span class="w-full border-t"></span>
      </div>
      <div class="relative flex justify-center text-xs uppercase">
        <span class="bg-card text-muted-foreground px-2"> Or continue with </span>
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
