import clone from "lodash-es/clone";
import { createQuery } from "@tanstack/svelte-query";
import { theAxios } from "$lib/theAxios";
import { encodeBase85 } from "base85";
import type { Person, UserWithCredential } from "$lib/types";
import type { LoginInput } from "$lib/schemas/auth";
import type { PartialDeep } from "type-fest";

export const getMe = async () => {
  const response = await theAxios.get<Person>("v0/me");
  return response.data;
};

export const login = async (loginInput: LoginInput) => {
  if (!loginInput?.username || !loginInput?.password) {
    throw new Error("Username and password are required");
  }
  // Encode username and password to base85
  const username = encodeBase85(clone(loginInput.username));
  const password = encodeBase85(clone(loginInput.password));

  const response = await theAxios.post<UserWithCredential>("v0/auth/login", {
    username,
    password,
  });
  return response.data;
};

export const logout = async () => {
  await theAxios.post("v0/auth/logout");
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
    queryKey: ["v0/me"],
    queryFn: getMe,
  });
