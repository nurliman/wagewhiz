<script lang="ts">
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

<style lang="postcss">
  :global(#nprogress) {
    @apply pointer-events-none;

    & :global(.bar) {
      @apply fixed left-0 top-0 z-[9999] w-full;
      background-color: var(--nprogress-color);
      height: var(--nprogress-height);
    }

    /* Fancy blur effect */
    & :global(.peg) {
      @apply absolute right-0 block h-full w-[100px] opacity-100;
      box-shadow:
        0 0 10px var(--nprogress-color),
        0 0 5px var(--nprogress-color);
      transform: rotate(3deg) translate3d(0px, -4px, 0);
    }

    /* Remove these to get rid of the spinner */
    & :global(.spinner) {
      @apply fixed right-[15px] top-[15px] z-[9999] block;
    }

    & :global(.spinner-icon) {
      @apply box-border h-[18px] w-[18px];
      @apply animate-spin duration-[400ms];

      border: solid 2px transparent;
      border-top-color: var(--nprogress-color);
      border-left-color: var(--nprogress-color);
      border-radius: 50%;
    }
  }

  :global(.nprogress-custom-parent) {
    @apply relative overflow-hidden;
  }

  :global(.nprogress-custom-parent #nprogress .spinner),
  :global(.nprogress-custom-parent #nprogress .bar) {
    @apply absolute;
  }
</style>
