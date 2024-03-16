import { theAxios } from "$lib/theAxios";
import { createQuery, createInfiniteQuery } from "@tanstack/svelte-query";
import type { PaginationParams, PaginationResponse, Person } from "$lib/types";

export const PEOPLE_ENDPOINT = "v0/people" as const;

export const getPeople = async (params: PaginationParams = {}) => {
  const response = await theAxios<PaginationResponse<Person>>(PEOPLE_ENDPOINT, { params });
  return response.data;
};

export const getPerson = async (id: string) => {
  if (!id) throw new Error("id is required");
  const response = await theAxios<Person>(`${PEOPLE_ENDPOINT}/${id}`);
  return response.data;
};

export const useGetPeopleQuery = (params: Omit<PaginationParams, "page"> = {}) => {
  return createInfiniteQuery({
    queryKey: [PEOPLE_ENDPOINT, params] as const,
    queryFn: ({ queryKey, pageParam }) =>
      getPeople({
        page: pageParam,
        page_size: queryKey[1].page_size,
      }),
    initialPageParam: 1,
    getNextPageParam: ({ page, page_size, total }) => {
      return page * page_size < total ? page + 1 : undefined;
    },
  });
};

export const useGetPersonQuery = (id: string) => {
  return createQuery({
    queryKey: [PEOPLE_ENDPOINT, id] as const,
    queryFn: ({ queryKey }) => getPerson(queryKey[1]),
  });
};
