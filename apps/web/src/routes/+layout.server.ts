import type { LayoutServerLoad } from "./$types.ts";

export const load = (async () => {
  return {
    currentDate: new Date(),
  };
}) satisfies LayoutServerLoad;
