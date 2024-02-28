<script lang="ts">
  import "../app.css";
  import ms from "ms";
  import { browser } from "$app/environment";
  import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query";
  import { ModeWatcher } from "mode-watcher";
  import { Toaster } from "$lib/components/ui/sonner";

  const title = "Wagewhiz";
  const description = "Open-source HR and Payroll software for small and medium-sized businesses.";

  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        enabled: browser,
        staleTime: ms("3 minutes"),
      },
    },
  });
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

<QueryClientProvider client={queryClient}>
  <ModeWatcher />
  <Toaster closeButton position="bottom-right" />

  <div class="flex min-h-full flex-col">
    <slot />
  </div>
</QueryClientProvider>
