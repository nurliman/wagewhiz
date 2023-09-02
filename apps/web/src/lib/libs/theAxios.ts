import endsWith from "lodash/endsWith";
import axios from "axios";
import axiosRetry from "axios-retry";
import { goto } from "$app/navigation";
import { theToast } from "$lib/libs/theToast";
import { refreshToken } from "$lib/apis/authApi";

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
    const refreshTokenData = await refreshToken().catch((refreshTokenError) => {
      if ([400, 401].includes(refreshTokenError.response?.status)) {
        let signInUrl = "/sign-in";

        if (window.location.href && !endsWith(window.location.href, "/sign-in")) {
          signInUrl += `?redirect=${window.location.href}`;
        }

        goto(signInUrl);

        theToast.error("Unauthorized, please sign in");
      }
    });
    if (!refreshTokenData?.credential?.access_token) return false;

    // TODO: update auth state here
    return true;
  },
});

export { theAxios };
