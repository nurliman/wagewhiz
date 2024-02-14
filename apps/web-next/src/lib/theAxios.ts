import startsWith from "lodash-es/startsWith";
import axios from "axios";
import axiosRetry from "axios-retry";
import { toast } from "svelte-sonner";
import { goto } from "$app/navigation";
import { refreshToken } from "$lib/api/auth";
import { isLoggedIn } from "$lib/stores/auth";

const BACKEND_BASE_URL = "http://localhost:3001";
const LOGIN_URL = "/login";

const theAxios = axios.create({
  baseURL: BACKEND_BASE_URL,
  timeout: 5000,
  paramsSerializer: {
    indexes: null,
  },
  headers: {
    "Content-Type": "application/json",
  },
  withCredentials: true,
});

// TODO: Replace axios with tanstack-query for retry and refresh token handling.
axiosRetry(theAxios, {
  retries: 3,
  shouldResetTimeout: true,
  retryDelay: axiosRetry.exponentialDelay,
  retryCondition: async (error) => {
    if (error.config?.url?.includes("sign-in") || error.config?.url?.includes("refresh-token")) {
      return axiosRetry.isNetworkError(error) || error.code === "ECONNABORTED";
    }
    if (error.response?.status !== 401) {
      return axiosRetry.isNetworkOrIdempotentRequestError(error) || error.code === "ECONNABORTED";
    }
    // TODO: make sure refresh token only called once at a time
    const refreshTokenData = await refreshToken().catch((refreshTokenError) => {
      if ([400, 401].includes(refreshTokenError.response?.status)) {
        let loginUrl = "" + LOGIN_URL; // clone the string

        if (window.location.href && !startsWith(window.location.pathname, LOGIN_URL)) {
          loginUrl += `?redirect=${window.location.href}`;
        }

        goto(loginUrl);

        toast.error("Unauthorized, please sign in");

        isLoggedIn.set(false);
      }
    });
    if (!refreshTokenData?.credential?.access_token) return false;

    isLoggedIn.set(true);

    return true;
  },
});

export { theAxios };
