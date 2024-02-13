<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import SpinnerPage from "$lib/components/SpinnerPage.svelte";

  onMount(() => {
    goto("/login");
  });
</script>

<SpinnerPage />
