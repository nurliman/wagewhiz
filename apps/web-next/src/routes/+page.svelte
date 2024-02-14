<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { isLoggedIn } from "$lib/stores/auth";
  import SpinnerPage from "$lib/components/SpinnerPage.svelte";

  onMount(() => {
    goto($isLoggedIn ? "/dashboard" : "/login");
  });
</script>

<SpinnerPage />
