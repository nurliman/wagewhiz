import * as v from "@valibot/valibot";

export const loginInputSchema = v.object({
  username: v.pipe(v.string(), v.minLength(1)),
  password: v.pipe(v.string(), v.minLength(1)),
});

export type LoginInput = v.InferOutput<typeof loginInputSchema>;
