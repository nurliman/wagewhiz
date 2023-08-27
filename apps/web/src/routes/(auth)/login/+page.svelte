<script lang="ts">
  import axios from "axios";
  import { goto } from "$app/navigation";
  import { nanoid } from "$lib/utils/nanoid.ts";
  import { createMutation } from "@tanstack/svelte-query";
  import { toastStore } from "@skeletonlabs/skeleton";
  import { superForm } from "sveltekit-superforms/client";
  import { signInInputSchema, type SignInInput } from "$lib/schemas/signInInputSchema.ts";
  import { signIn } from "$lib/apis/authApi.ts";
  import type { PageData } from "./$types.ts";
  import type { UserWithCredential } from "$lib/types.ts";

  export let data: PageData;
  const fieldUsernameId = nanoid();
  const fieldPasswordId = nanoid();

  const signInMutation = createMutation<UserWithCredential, Error, SignInInput>({
    mutationFn: signIn,
    onSuccess: () => {
      // TODO: save credentials to persistent storage here

      const query = new URLSearchParams(window.location.search);
      const redirectUrl = query?.get?.("redirect") || "/dashboard";
      goto(redirectUrl);
    },
    onError: (err) => {
      if (axios.isAxiosError(err)) {
        if (err.response?.data?.message) {
          toastStore.trigger({
            message: err.response.data.message,
            background: "variant-filled-error",
          });
        } else {
          toastStore.trigger({
            message: "An error occurred",
            background: "variant-filled-error",
          });
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

<section class="bg-gray-50 dark:bg-gray-900">
  <div class="flex flex-col items-center justify-center px-6 py-8 mx-auto md:h-screen lg:py-0">
    <a href="#" class="flex items-center mb-6 text-2xl font-semibold text-gray-900 dark:text-white">
      <img
        class="w-8 h-8 mr-2"
        src="https://flowbite.s3.amazonaws.com/blocks/marketing-ui/logo.svg"
        alt="logo"
      />
      Flowbite
    </a>
    <div
      class="w-full bg-white rounded-lg shadow dark:border md:mt-0 sm:max-w-md xl:p-0 dark:bg-gray-800 dark:border-gray-700"
    >
      <div class="p-6 space-y-4 md:space-y-6 sm:p-8">
        <h1
          class="text-xl font-bold leading-tight tracking-tight text-gray-900 md:text-2xl dark:text-white"
        >
          Sign in to your account
        </h1>
        <form class="space-y-4 md:space-y-6" method="POST" use:enhance>
          <div>
            <label
              for={fieldUsernameId}
              class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
            >
              Username
            </label>
            <input
              id={fieldUsernameId}
              name="username"
              type="text"
              class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
              placeholder="Username"
              bind:value={$form.username}
              {...$constraints.username}
            />
          </div>
          <div>
            <label
              for={fieldPasswordId}
              class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
            >
              Password
            </label>
            <input
              type="password"
              name="password"
              id={fieldPasswordId}
              placeholder="••••••••"
              class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
              bind:value={$form.password}
              {...$constraints.password}
            />
          </div>
          <div class="flex items-center justify-between">
            <div class="flex items-start">
              <div class="flex items-center h-5">
                <input
                  id="remember"
                  aria-describedby="remember"
                  type="checkbox"
                  class="w-4 h-4 border border-gray-300 rounded bg-gray-50 focus:ring-3 focus:ring-primary-300 dark:bg-gray-700 dark:border-gray-600 dark:focus:ring-primary-600 dark:ring-offset-gray-800"
                  required=""
                />
              </div>
              <div class="ml-3 text-sm">
                <label for="remember" class="text-gray-500 dark:text-gray-300">Remember me</label>
              </div>
            </div>
            <a
              href="#"
              class="text-sm font-medium text-primary-600 hover:underline dark:text-primary-500"
              >Forgot password?</a
            >
          </div>
          <button
            type="submit"
            class="w-full text-white bg-primary-600 hover:bg-primary-700 focus:ring-4 focus:outline-none focus:ring-primary-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-primary-600 dark:hover:bg-primary-700 dark:focus:ring-primary-800"
            >Sign in</button
          >
          <p class="text-sm font-light text-gray-500 dark:text-gray-400">
            Don’t have an account yet? <a
              href="#"
              class="font-medium text-primary-600 hover:underline dark:text-primary-500">Sign up</a
            >
          </p>
        </form>
      </div>
    </div>
  </div>
</section>
