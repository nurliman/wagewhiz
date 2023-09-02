<script lang="ts">
  import { isAxiosError } from "axios";
  import { goto } from "$app/navigation";
  import { nanoid } from "$lib/utils/nanoid.ts";
  import { createMutation } from "@tanstack/svelte-query";
  import { superForm } from "sveltekit-superforms/client";
  import { signInInputSchema, type SignInInput } from "$lib/schemas/signInInputSchema.ts";
  import { signIn } from "$lib/apis/authApi.ts";
  import { theToast } from "$lib/libs/theToast";
  import Spinner from "$lib/components/Spinner.svelte";
  import type { PageData } from "./$types.ts";
  import type { UserWithCredential } from "$lib/types.ts";

  export let data: PageData;
  const fieldUsernameId = nanoid();
  const fieldPasswordId = nanoid();

  const signInMutation = createMutation<UserWithCredential, Error, SignInInput>({
    mutationFn: signIn,
    onSuccess: async () => {
      // TODO: save credentials to persistent storage here

      const query = new URLSearchParams(window.location.search);
      const redirectUrl = query?.get?.("redirect") || "/dashboard";
      await goto(redirectUrl);
    },
    onError: (err) => {
      if (isAxiosError(err)) {
        if (err.response?.data?.message) {
          theToast.error(err.response.data.message);
        } else {
          theToast.error("An error occurred");
        }
      }

      console.error(err);
    },
  });

  const { form, constraints, enhance } = superForm(data.form, {
    SPA: true,
    validators: signInInputSchema,
    onUpdate({ form }) {
      if (form.valid) {
        $signInMutation.mutate(form.data);
      }
    },
  });
</script>

<section>
  <div class="flex flex-col items-center justify-center px-6 py-8 mx-auto md:h-screen lg:py-0">
    <a href="/" class="flex items-center mb-6 text-2xl font-semibold text-gray-900 dark:text-white">
      {"[ WageWhiz ]"}
    </a>
    <div class="card w-full sm:max-w-md">
      <div class="p-6 space-y-4 md:space-y-6 sm:p-8">
        <h1 class="h3 mb-4 font-bold">Sign in to your account</h1>
        <form class="space-y-4 md:space-y-6" method="POST" use:enhance>
          <div class="space-y-2">
            <label for={fieldUsernameId} class="label"> Username </label>
            <input
              id={fieldUsernameId}
              name="username"
              type="text"
              class="input"
              placeholder="Username"
              bind:value={$form.username}
              {...$constraints.username}
            />
          </div>
          <div class="space-y-2">
            <label for={fieldPasswordId} class="label"> Password </label>
            <input
              type="password"
              name="password"
              id={fieldPasswordId}
              placeholder="••••••••"
              class="input"
              bind:value={$form.password}
              {...$constraints.password}
            />
          </div>
          <div class="flex items-center justify-between">
            <div class="flex items-start">
              <label class="flex items-center space-x-2">
                <input class="checkbox checked:!bg-secondary-500" type="checkbox" />
                <span class="text-sm">Remember me</span>
              </label>
            </div>
            <a
              href="/forgot-password"
              class="text-sm font-medium text-secondary-600 hover:underline"
            >
              Forgot password?
            </a>
          </div>
          <button
            type="submit"
            class="w-full btn variant-filled-primary"
            disabled={$signInMutation.status === "loading"}
          >
            <div>Sign in</div>
            {#if $signInMutation.status === "loading"}
              <Spinner containerClass="w-4 h-4 ml-2" class="!border-[3px]" />
            {/if}
          </button>
          <p class="text-sm font-light text-gray-500 dark:text-gray-400">
            <span>Don’t have an account yet?&nbsp;</span>
            <a href="/sign-up" class="font-medium text-secondary-600 hover:underline">Sign up</a>
          </p>
        </form>
      </div>
    </div>
  </div>
</section>
