import { superValidate } from "sveltekit-superforms/client";
import { signInInputSchema } from "$lib/schemas/signInInputSchema.ts";
import type { PageLoad } from "./$types.ts";

export const load = (async () => {
  const form = await superValidate(signInInputSchema);
  return { form };
}) satisfies PageLoad;
