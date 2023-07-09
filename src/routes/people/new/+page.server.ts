import { fail } from "@sveltejs/kit";
import { superValidate } from "sveltekit-superforms/server";
import { newPersonInputSchema } from "$lib/schemas/newPersonInputSchema.ts";
import type { Actions, PageServerLoad } from "./$types.ts";

export const load: PageServerLoad = async () => {
  const form = await superValidate(newPersonInputSchema);
  return { form };
};

export const actions: Actions = {
  default: async ({ request }) => {
    const formData = await request.formData();
    const form = await superValidate(formData, newPersonInputSchema);
    const avatar = formData.get("avatar");

    console.log("POST", { ...form });

    if (!form.valid) {
      return fail(400, { form });
    }

    if (avatar && !(avatar instanceof File)) {
      // @ts-ignore
      form.errors.avatar = "Avatar must be a file";
      return fail(400, { form });
    }

    return { form };
  },
};
