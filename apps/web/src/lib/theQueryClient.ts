import { browser } from "$app/environment";
import { QueryClient } from "@tanstack/svelte-query";
import ms from "ms";

export const theQueryClient = new QueryClient({
  defaultOptions: {
    queries: {
      enabled: browser,
      staleTime: ms("3 minutes"),
      retry: false,
    },
  },
});
