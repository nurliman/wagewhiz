import { graphql } from "$lib/graphql";
import { execute } from "$lib/graphql/execute";
import type { LoginInput } from "$lib/graphql/graphql";
import { encodeBase85 } from "base85";
import clone from "lodash-es/clone";

export const loginMutation = graphql(`
  mutation Login($input: LoginInput!) {
    login(input: $input) {
      user {
        id
        createdAt
        updatedAt
        username
        role
        personId
      }
      credential {
        accessToken
        refreshToken
      }
    }
  }
`);

export const login = async ({ username, password }: LoginInput) => {
  if (!username || !password) {
    throw new Error("Username and password are required");
  }
  // Encode username and password to base85
  username = encodeBase85(clone(username));
  password = encodeBase85(clone(password));

  return execute(loginMutation, { input: { username, password } }).then((res) => res.login);
};

export const logoutMutation = graphql(`
  mutation Logout {
    logout
  }
`);

export const logout = async () => {
  return execute(logoutMutation).then((res) => res.logout);
};

export const refreshTokenMutation = graphql(`
  mutation RefreshToken {
    refreshToken {
      user {
        id
        createdAt
        updatedAt
        username
        role
        personId
      }
      credential {
        accessToken
        refreshToken
      }
    }
  }
`);

export const refreshToken = async () => {
  return execute(refreshTokenMutation);
};
