<script lang="ts">
  import isBoolean from "lodash-es/isBoolean";
  import { onMount, onDestroy } from "svelte";
  import { goto } from "$app/navigation";
  import { isLoggedIn } from "$lib/stores/auth";
  import SpinnerPage from "$lib/components/SpinnerPage.svelte";
  import Footer from "$lib/components/Footer.svelte";

  let isLoggedInState = $state<boolean | null>(null);

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

{#if isBoolean(isLoggedInState) && !isLoggedInState}
  <div class="flex h-full w-full flex-1 flex-col">
    <main class="flex flex-1 flex-col py-6 lg:py-8">
      <slot />
    </main>

    <Footer />
  </div>
{:else}
  <SpinnerPage />
{/if}
