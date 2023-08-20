import { browser } from "$app/environment";
import { QueryClient } from "@tanstack/svelte-query";

export const theQueryClient = new QueryClient({
  defaultOptions: {
    queries: {
      enabled: browser,
    },
  },
});
