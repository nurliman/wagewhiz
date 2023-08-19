import { z } from "zod";

export const signInInputSchema = z.object({
  username: z.string().min(1),
  password: z.string().min(1),
});

export type SignInInput = z.infer<typeof signInInputSchema>;
