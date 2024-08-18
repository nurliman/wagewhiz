<script>
  import "@fontsource-variable/plus-jakarta-sans";
  import "../app.css";
  import { theQueryClient } from "$lib/theQueryClient";
  import { ModeWatcher } from "mode-watcher";
  import { QueryClientProvider } from "@tanstack/svelte-query";
  import { Toaster } from "$lib/components/ui/sonner";
  import NProgress from "$lib/components/NProgress.svelte";

  // TODO: change title and description to be dynamic
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

<QueryClientProvider client={theQueryClient}>
  <ModeWatcher />

  <NProgress />

  <Toaster closeButton position="bottom-right" />

  <div class="flex min-h-full flex-col">
    <slot />
  </div>
</QueryClientProvider>
