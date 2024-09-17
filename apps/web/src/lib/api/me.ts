import { graphql } from "$lib/graphql";
import { execute } from "$lib/graphql/execute";
import { createQuery } from "@tanstack/svelte-query";
import ms from "ms";

export const meQuery = graphql(`
  query GetMe {
    me {
      id
      createdAt
      updatedAt
      username
      role
      personId
    }
  }
`);

export const getMe = async () => {
  return execute(meQuery).then((res) => res.me);
};

export const useGetMeQuery = () => {
  return createQuery({
    queryKey: ["v0/me"],
    queryFn: getMe,
    staleTime: 0,
    refetchInterval: ms("5 minutes"),
  });
};
