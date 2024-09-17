import { goto } from "$app/navigation";
import { loginMutation, refreshToken, refreshTokenMutation } from "$lib/api";
import { AppError, isAppError } from "$lib/errors";
import { accessToken as $accessToken, isLoggedIn } from "$lib/stores/auth";
import axios from "axios";
import { nanoid } from "nanoid";
import pRetry from "p-retry";
import { toast } from "svelte-sonner";
import type { TypedDocumentString } from "./graphql";

// TODO: move to env
const endpoint = "http://localhost:3001/graphql";

const graphqlAxios = axios.create({
  baseURL: endpoint,
  method: "POST",
  headers: {
    "Content-Type": "application/json",
    Accept: "application/json",
  },
  withCredentials: true,
});

// Request interceptor to set the X-Request-ID header
graphqlAxios.interceptors.request.use((config) => {
  config.headers["X-Request-ID"] = nanoid();
  return config;
});

// Request interceptor to set the Authorization header
graphqlAxios.interceptors.request.use((config) => {
  const accessToken = $accessToken.get();
  if (accessToken) {
    config?.headers?.set?.("Authorization", `Bearer ${accessToken}`);
  }
  return config;
});

graphqlAxios.interceptors.response.use((response) => {
  // Check if response.data is an object
  if (typeof response.data !== "object" || response.data === null) {
    return Promise.reject(new AppError("Unexpected response format"));
  }

  const gqlResponse = response.data;

  if (gqlResponse.errors?.length) {
    // If there are GraphQL errors, reject the promise
    return Promise.reject(
      new AppError(gqlResponse.errors[0].message, gqlResponse.errors[0].extensions?.code),
    );
  }

  // Only return the data field when there are no errors
  response.data = gqlResponse.data || {};

  return response;
});

const LOGIN_PAGE_PATH = "/login";

const handleUnauthenticated = async (query: TypedDocumentString<any, any>) => {
  if (query === refreshTokenMutation || query === loginMutation) return false;

  try {
    const refreshTokenData = await refreshToken();
    if (refreshTokenData?.refreshToken?.credential?.accessToken) {
      isLoggedIn.set(true);
      return true;
    }
  } catch (refreshTokenError) {
    if (
      isAppError(refreshTokenError) &&
      ["UNAUTHENTICATED", "FORBIDDEN"].includes(refreshTokenError.code)
    ) {
      redirectToLogin();
    }
  }
  return false;
};

const redirectToLogin = () => {
  const loginPagePath = new URL(LOGIN_PAGE_PATH, window.location.origin);
  if (!window.location.pathname.startsWith(LOGIN_PAGE_PATH)) {
    loginPagePath.searchParams.set("redirect", window.location.href);
  }
  isLoggedIn.set(false);
  goto(loginPagePath.toString());
  toast.error("Unauthorized, please login again.");
};

export const execute = async <TResult, TVariables>(
  query: TypedDocumentString<TResult, TVariables>,
  ...[variables]: TVariables extends Record<string, never> ? [] : [TVariables]
): Promise<TResult> => {
  const response = await pRetry(
    () => {
      return graphqlAxios<TResult>({
        method: "POST",
        data: {
          query,
          variables,
        },
      });
    },
    {
      retries: 2,
      shouldRetry: async (err) => {
        if (!isAppError(err)) return true;

        switch (err.code) {
          case "UNAUTHENTICATED":
            return await handleUnauthenticated(query);
          case "INTERNAL_SERVER_ERROR":
          case "UNKNOWN_ERROR":
            return true;
          default:
            return false;
        }
      },
    },
  ).catch((error) => {
    if (isAppError(error)) {
      console.error("GraphQL Error:", error.message, error.code);
    } else {
      console.error("Unexpected error:", error);
    }
    throw error;
  });

  return response?.data;
};
