<script lang="ts">
  import isBoolean from "lodash-es/isBoolean";
  import { onMount, onDestroy } from "svelte";
  import { goto } from "$app/navigation";
  import { isLoggedIn } from "$lib/stores/auth";
  import SpinnerPage from "$lib/components/SpinnerPage.svelte";

  let isLoggedInState: boolean | null = null;

  onMount(() => {
    // check if user is already logged in
    if ($isLoggedIn) {
      return goto("/dashboard");
    }

    isLoggedInState = isBoolean($isLoggedIn) ? $isLoggedIn : false;
  });

  onDestroy(() => {
    isLoggedInState = null;
  });
</script>

<main class="mx-auto flex h-full w-full max-w-xl flex-1 flex-col justify-center p-4 md:p-10">
  {#if isBoolean(isLoggedInState) && !isLoggedInState}
    <slot />
  {:else}
    <SpinnerPage />
  {/if}
</main>
