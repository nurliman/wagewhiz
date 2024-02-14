import { createQuery } from "@tanstack/svelte-query";
import { theAxios } from "$lib/theAxios";
import type { Person, UserWithCredential } from "$lib/types.ts";
import type { LoginInput } from "$lib/schemas/auth";
import type { PartialDeep } from "type-fest";

export const getMe = async () => {
  const response = await theAxios.get<Person>("v0/me");
  return response.data;
};

export const login = async (signInInput: LoginInput) => {
  const response = await theAxios.post<UserWithCredential>("v0/auth/sign-in", signInInput);
  return response.data;
};

export const logout = async () => {
  await theAxios.post("v0/auth/sign-out");
  return null;
};

export const refreshToken = async (refreshToken?: string) => {
  const response = await theAxios.post<PartialDeep<UserWithCredential>>("v0/auth/refresh-token", {
    refresh_token: refreshToken,
  });
  return response.data;
};

export const useGetMeQuery = () =>
  createQuery<Person, Error>({
    queryKey: ["me"],
    queryFn: getMe,
  });
