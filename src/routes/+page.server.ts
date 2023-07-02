import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types.ts";

export const load = (async () => {
  throw redirect(301, "/dashboard");
}) satisfies PageServerLoad;
