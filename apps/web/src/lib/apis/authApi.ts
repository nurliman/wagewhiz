import { createQuery } from "@tanstack/svelte-query";
import { theAxios } from "$lib/libs/theAxios.ts";
import type { Person } from "$lib/types.ts";

export const getMe = async () => {
  const response = await theAxios.get<Person>("v0/me");
  return response.data;
};

export const getMeQuery = () =>
  createQuery<Person, Error>({
    queryKey: ["me"],
    queryFn: getMe,
  });
