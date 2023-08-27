import endsWith from "lodash/endsWith";
import axios from "axios";
import axiosRetry from "axios-retry";
import { goto } from "$app/navigation";
import type { UserWithCredential } from "$lib/types";
import type { PartialDeep } from "type-fest";

const BACKEND_BASE_URL = "http://localhost:3001";

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
    const res = await theAxios
      .post<PartialDeep<UserWithCredential>>(`${BACKEND_BASE_URL}/v0/auth/refresh-token`, {})
      .catch((refreshTokenError) => {
        if ([400, 401].includes(refreshTokenError.response?.status)) {
          let loginUrl = "/login";

          if (window.location.href && !endsWith(window.location.href, "/login")) {
            loginUrl += `?redirect=${window.location.href}`;
          }

          goto(loginUrl);
        }
      });
    if (!res?.data?.credential?.access_token) return false;

    // TODO: update auth state here
    return true;
  },
});

export { theAxios };
