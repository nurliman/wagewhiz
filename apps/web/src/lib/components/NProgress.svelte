<script lang="ts">
  import "./NProgress.css";
  import isObject from "lodash-es/isObject";
  import { onMount, onDestroy } from "svelte";
  import { browser } from "$app/environment";
  import { afterNavigate, beforeNavigate } from "$app/navigation";
  import NProgress, { type NProgressOptions } from "nprogress";

  type NProgressProps = {
    height?: number | string;
    color?: string;
    colorDark?: string;
    delay?: number;
    options?: Partial<NProgressOptions>;
    nonce?: string;
    disableSameRoute?: boolean;
  };

  export let height: NProgressProps["height"] = "2px";
  export let color: NProgressProps["color"] = "#f540cc";
  export let colorDark: NProgressProps["colorDark"];
  export let delay: NProgressProps["delay"] = 0;
  export let disableSameRoute: NProgressProps["disableSameRoute"] = true;
  export let nonce: NProgressProps["nonce"] = "nprogress";
  export let options: NProgressProps["options"] = {};

  isObject(options) && (options.showSpinner ??= false);

  let timer: ReturnType<typeof setTimeout>;
  let incInterval: ReturnType<typeof setInterval>;
  let styleElement: HTMLStyleElement;

  $: stylesheet = `
    :root {
      --nprogress-color:${color};
      --nprogress-height:${height};
    }

    ${colorDark ? `:root.dark{--nprogress-color:${colorDark};}` : ""}
  `;

  onMount(() => {
    isObject(options) && NProgress.configure(options);

    return () => {
      clearTimeout(timer);
      clearInterval(incInterval);
      NProgress.remove();
    };
  });

  beforeNavigate((nav) => {
    clearTimeout(timer);
    clearInterval(incInterval);
    if (disableSameRoute && nav.from?.route.id && nav.from?.route.id === nav.to?.route.id) return;
    timer = setTimeout(NProgress.start, delay);
    incInterval = setInterval(NProgress.inc, 1000);
  });

  afterNavigate(() => {
    setTimeout(() => {
      clearTimeout(timer);
      clearInterval(incInterval);
      NProgress.done();
    }, 1);
  });

  $: {
    isObject(options) && NProgress.configure(options);
  }

  $: {
    if (browser) {
      styleElement?.remove?.();
      styleElement = document.createElement("style");
      styleElement.setAttribute("nonce", nonce || "");
      styleElement.innerHTML = stylesheet;
      document.head.appendChild(styleElement);
    }
  }

  onDestroy(() => {
    browser && styleElement?.remove?.();
  });
</script>
