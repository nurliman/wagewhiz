import { theAxios } from "$lib/theAxios";
import { createQuery } from "@tanstack/svelte-query";
import type { Person } from "$lib/types";

export const getPeople = async () => {
  const response = await theAxios.get<Person[]>("v0/people");
  return response.data;
};

export const useGetPeopleQuery = () =>
  createQuery<Person[], Error>({
    queryKey: ["v0/people"],
    queryFn: getPeople,
  });
