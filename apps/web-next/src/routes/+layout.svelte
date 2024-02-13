<script lang="ts">
  import "../app.css";

  const title = "Wagewhiz";
  const description = "Open-source HR and Payroll software for small and medium-sized businesses.";
</script>

<svelte:head>
  <title>{title}</title>
  <meta name="description" content={description} />

  <!-- Open Graph -->
  <meta property="og:type" content="website" />
  <meta property="og:title" content={title} />
  <meta property="og:site_name" content={title?.split?.(" | ")?.[0] || title} />
  <meta property="og:description" content={description} />
  <!-- TODO: Add og:image -->

  <!-- Twitter -->
  <meta name="twitter:title" content={title} />
  <meta name="twitter:description" content={description} />
  <!-- TODO: Add twitter:domain -->
  <!-- TODO: Add twitter:image -->
</svelte:head>

<div class="flex min-h-full flex-col">
  <main class="flex flex-1 flex-col">
    <slot />
  </main>
</div>
