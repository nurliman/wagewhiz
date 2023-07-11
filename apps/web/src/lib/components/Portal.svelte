<script context="module" lang="ts">
  import { tick } from "svelte";

  /**
   * Usage:
   * ```js
   * <div use:portal={'css selector'}>
   * ```
   * or
   * ```js
   * <div use:portal={document.body}>
   * ```
   *
   * @param el
   * @param target DOM Element or CSS Selector
   */
  export const portal = (el: HTMLElement, target: HTMLElement | string = "body") => {
    let targetEl: HTMLElement | null;
    const update = async (newTarget: HTMLElement | string) => {
      target = newTarget;
      if (typeof target === "string") {
        targetEl = document.querySelector(target);
        if (targetEl === null) {
          await tick();
          targetEl = document.querySelector(target);
        }
        if (targetEl === null) {
          throw new Error(`No element found matching css selector: "${target}"`);
        }
      } else if (target instanceof HTMLElement) {
        targetEl = target;
      } else {
        throw new TypeError(
          `Unknown portal target type: ${
            target === null ? "null" : typeof target
          }. Allowed types: string (CSS selector) or HTMLElement.`,
        );
      }
      targetEl.appendChild(el);
      el.hidden = false;
    };

    const destroy = () => {
      if (el.parentNode) {
        el.parentNode.removeChild(el);
      }
    };

    update(target);

    return {
      update,
      destroy,
    };
  };
</script>

<script lang="ts">
  /**
   * DOM Element or CSS Selector
   */
  export let target: HTMLElement | string = "body";
</script>

<div use:portal={target} hidden>
  <slot />
</div>
