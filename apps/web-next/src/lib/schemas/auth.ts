import {
  object as vObject,
  string as vString,
  minLength as vMinLength,
  type Output,
} from "valibot";

export const loginInputSchema = vObject({
  username: vString([vMinLength(1)]),
  password: vString([vMinLength(1)]),
});

export type LoginInput = Output<typeof loginInputSchema>;
