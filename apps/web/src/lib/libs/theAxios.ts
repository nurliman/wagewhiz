import axios from "axios";
import _axiosRetry from "axios-retry";

const axiosRetry = _axiosRetry as unknown as typeof _axiosRetry.default;

const theAxios = axios.create({
  // TODO: use env var
  baseURL: "http://localhost:3001",
});

axiosRetry(theAxios, { retries: 3 });

// TODO: handle refresh token

export { theAxios };
