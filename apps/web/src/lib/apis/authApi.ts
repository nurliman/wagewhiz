import { createQuery } from "@tanstack/svelte-query";
import { theAxios } from "$lib/libs/theAxios.ts";
import type { Person, UserWithCredential } from "$lib/types.ts";
import type { SignInInput } from "$lib/schemas/signInInputSchema";
import type { PartialDeep } from "type-fest";

export const getMe = async () => {
  const response = await theAxios.get<Person>("v0/me");
  return response.data;
};

export const signIn = async (signInInput: SignInInput) => {
  const response = await theAxios.post<UserWithCredential>("v0/auth/sign-in", signInInput);
  return response.data;
};

export const refreshToken = async (refreshToken?: string) => {
  const response = await theAxios.post<PartialDeep<UserWithCredential>>("v0/auth/refresh-token", {
    refresh_token: refreshToken,
  });
  return response.data;
};

export const getMeQuery = () =>
  createQuery<Person, Error>({
    queryKey: ["me"],
    queryFn: getMe,
  });
