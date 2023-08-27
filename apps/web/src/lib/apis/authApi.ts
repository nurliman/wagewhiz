import { createQuery } from "@tanstack/svelte-query";
import { theAxios } from "$lib/libs/theAxios.ts";
import type { Person, UserWithCredential } from "$lib/types.ts";
import type { SignInInput } from "$lib/schemas/signInInputSchema";

export const getMe = async () => {
  const response = await theAxios.get<Person>("v0/me");
  return response.data;
};

export const signIn = async (signInInput: SignInInput) => {
  const response = await theAxios.post<UserWithCredential>("v0/auth/sign-in", signInInput);
  return response.data;
};

export const getMeQuery = () =>
  createQuery<Person, Error>({
    queryKey: ["me"],
    queryFn: getMe,
  });
