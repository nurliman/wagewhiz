<script>
  import Spinner from "$lib/components/Spinner.svelte";
</script>

<div class="flex h-full justify-center items-center flex-1">
  <Spinner client:only="svelte" containerClass="w-16 h-16" />
</div>
