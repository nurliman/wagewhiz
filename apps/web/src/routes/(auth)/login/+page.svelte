<script lang="ts">
  import { superForm, defaults } from "sveltekit-superforms";
  import { valibot } from "sveltekit-superforms/adapters";
  import { createMutation } from "@tanstack/svelte-query";
  import { toast } from "svelte-sonner";
  import { isLoggedIn } from "$lib/stores/auth";
  import { login } from "$lib/api";
  import { loginInputSchema } from "$lib/schemas/auth";
  import { goto } from "$app/navigation";
  import { cn } from "$lib/utils/shadcn";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import * as Card from "$lib/components/ui/card";
  import * as Form from "$lib/components/ui/form";
  import FluentSpinnerIos16Regular from "virtual:icons/fluent/spinner-ios-16-regular";
  import { isAppError } from "$lib/errors";

  const loginMutation = createMutation({
    mutationFn: login,
    onSuccess: async () => {
      $isLoggedIn = true;

      const query = new URLSearchParams(window.location.search);
      const redirectUrl = query?.get?.("redirect") || "/dashboard";
      await goto(redirectUrl);
    },
    onError: (err) => {
      let message = "An error occurred, please try again.";
      if (isAppError(err)) {
        message = err.message;
      }
      toast.error(message);
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

<div class="container flex max-w-md flex-1 flex-col">
  <div class="mx-auto flex w-full max-w-xl flex-1 flex-col justify-center">
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
            <Form.Button
              class="flex w-full flex-row items-center"
              disabled={$loginMutation.isPending}
            >
              <FluentSpinnerIos16Regular
                class={cn("mr-2 animate-spin", !$loginMutation.isPending && "hidden")}
              />
              Login
            </Form.Button>
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
          <Button
            class="flex-1"
            type="button"
            variant="outline"
            disabled={$loginMutation.isPending}
            on:click={() => toast.error("Not implemented yet.")}
          >
            <!-- TODO: Add GitHub icon -->
            GitHub
          </Button>
          <Button
            class="flex-1"
            type="button"
            variant="outline"
            disabled={$loginMutation.isPending}
            on:click={() => toast.error("Not implemented yet.")}
          >
            <!-- TODO: Add Google icon -->
            Google
          </Button>
        </div>
      </Card.Content>
    </Card.Root>
  </div>
</div>
