import { graphql } from "$lib/graphql";
import { execute } from "$lib/graphql/execute";
import type { PaginationParams } from "$lib/types";
import { createInfiniteQuery, createQuery } from "@tanstack/svelte-query";

export const PEOPLE_QUERY_KEY = "people" as const;

export const peopleQuery = graphql(`
  query GetPeople($page: Int, $pageSize: Int) {
    people(page: $page, pageSize: $pageSize) {
      items {
        id
        createdAt
        updatedAt
        name
        nik
        country
        city
        address
        zipCode
        email
        phone
        birthday
        organization
        role
        department
        joiningDate
        isActive
        gender
        status
        avatarUrl
      }
      page
      pageSize
      total
    }
  }
`);

export const getPeople = async (params: PaginationParams = {}) => {
  return execute(peopleQuery, params).then((res) => res.people);
};

export const useGetPeopleQuery = (params: Omit<PaginationParams, "page"> = {}) => {
  return createInfiniteQuery({
    queryKey: [PEOPLE_QUERY_KEY, params] as const,
    queryFn: ({ queryKey, pageParam }) =>
      getPeople({
        page: pageParam,
        pageSize: queryKey[1].pageSize,
      }),
    initialPageParam: 1,
    getNextPageParam: ({ page, pageSize, total }) => {
      return page * pageSize < total ? page + 1 : undefined;
    },
  });
};

export const personByIdQuery = graphql(`
  query GetPersonById($id: UUID!) {
    person(id: $id) {
      id
      createdAt
      updatedAt
      name
      nik
      country
      city
      address
      zipCode
      email
      phone
      birthday
      organization
      role
      department
      joiningDate
      isActive
      gender
      status
      avatarUrl
    }
  }
`);

export const getPerson = async (id: string) => {
  if (!id) throw new Error("id is required");
  return execute(personByIdQuery, { id }).then((res) => res.person);
};

export const useGetPersonQuery = (id: string) => {
  return createQuery({
    queryKey: [PEOPLE_QUERY_KEY, id] as const,
    queryFn: ({ queryKey }) => getPerson(queryKey[1]),
  });
};
