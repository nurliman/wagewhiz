<script lang="ts">
  import { onMount } from "svelte";
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
    color = "#f540cc",
    colorDark,
    delay = 0,
    disableSameRoute = true,
    nonce = "nprogress",
    options = {},
  } = $props<NProgressProps>();

  options.showSpinner ??= false;

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
    NProgress.configure(options);

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

  $effect.pre(() => {
    const style = document.createElement("style");
    style.setAttribute("nonce", nonce);
    style.innerHTML = stylesheet;
    document.head.appendChild(style);

    return () => {
      document.querySelector("style[nonce=nprogress]")?.remove?.();
    };
  });

  $effect(() => {
    NProgress.configure(options);
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
