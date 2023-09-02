import merge from "deepmerge";
import { toast } from "svelte-toast";
import { isPOJO } from "$lib/utils/isPOJO";
import type { SvelteToastOptions } from "svelte-toast/stores";

type TheToastArgs = Parameters<typeof toast.push>;

const theToast = (...args: TheToastArgs) => toast.push(...args);

theToast.pop = toast.pop.bind(toast);

theToast.error = (...args: TheToastArgs) => {
  const opts: SvelteToastOptions = {
    theme: {
      "--toastBackground": "rgb(var(--color-error-500))",
      "--toastBarBackground": "rgb(var(--color-error-800))",
    },
  };

  if (typeof args[0] === "string") {
    args[1] = isPOJO(args[1]) ? merge(args[1], opts) : opts;
  } else if (isPOJO(args[0])) {
    args[0] = merge(args[0], opts);
  }

  return toast.push(...args);
};

export { theToast };
