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

  let {
    height = "2px",
    color = "hsl(var(--secondary) / 1)",
    colorDark,
    delay = 0,
    disableSameRoute = true,
    nonce = "nprogress",
    options = {},
  }: NProgressProps = $props();

  isObject(options) && (options.showSpinner ??= false);

  let timer = $state<ReturnType<typeof setTimeout>>();
  let incInterval = $state<ReturnType<typeof setInterval>>();

  let stylesheet = $derived(`
    :root {
      --nprogress-color:${color};
      --nprogress-height:${height};
    }

    ${colorDark ? `:root.dark{--nprogress-color:${colorDark};}` : ""}
  `);

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

  $effect(() => {
    isObject(options) && NProgress.configure(options);
  });

  $effect.pre(() => {
    let styleElement: HTMLStyleElement | undefined;

    styleElement?.remove?.();
    styleElement = document.createElement("style");
    styleElement.setAttribute("nonce", nonce || "");
    styleElement.innerHTML = stylesheet;
    document.head.appendChild(styleElement);

    return () => {
      styleElement?.remove?.();
    };
  });
</script>
