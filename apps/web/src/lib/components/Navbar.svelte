<script lang="ts">
  import { goto } from "$app/navigation";
  import { Avatar, LightSwitch, popup } from "@skeletonlabs/skeleton";
  import { createMutation } from "@tanstack/svelte-query";
  import { toastStore } from "@skeletonlabs/skeleton";
  import { isAxiosError } from "axios";
  import { nanoid } from "$lib/utils/nanoid.ts";
  import { toggleSidebar } from "$lib/stores/sidebar.ts";
  import { signOut } from "$lib/apis/authApi.ts";
  import type { PopupSettings } from "@skeletonlabs/skeleton";

  const avatarPopupId = nanoid();
  const avatarPopup: PopupSettings = {
    event: "click",
    target: avatarPopupId,
    placement: "bottom",
  };

  const signOutMutation = createMutation<null, Error>({
    mutationFn: signOut,
    onSuccess: async () => {
      // TODO: update persistent storage here
      await goto("/sign-in");
    },
    onError: (err) => {
      if (isAxiosError(err)) {
        if (err.response?.data?.message) {
          toastStore.trigger({
            message: err.response.data.message,
            background: "variant-filled-error",
          });
        } else {
          toastStore.trigger({
            message: "An error occurred",
            background: "variant-filled-error",
          });
        }
      }

      console.error(err);
    },
  });
</script>

<!-- TODO: handle sign out loading state -->

<nav class="navbar">
  <div class="navbar-inner">
    <div class="flex items-center">
      <button
        type="button"
        class="btn-icon btn-icon-sm variant-soft rounded-lg p-1.5 ml-1.5 lg:hidden"
        on:click={toggleSidebar}
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="20"
          height="20"
          class="h-6 w-6 shrink-0"
          aria-label="bars 3"
          aria-hidden="true"
          fill="none"
          viewBox="0 0 20 20"
          stroke-width="2"
          role="button"
        >
          <path
            fill="currentColor"
            clip-rule="evenodd"
            fill-rule="evenodd"
            d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h6a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z"
          />
        </svg>
      </button>
      <a href="/" class="items-center flex ml-3.5 lg:ml-2 md:mr-24">
        <span class="self-center whitespace-nowrap text-xl font-semibold">[ WageWhiz ]</span>
      </a>
    </div>
    <div class="flex items-center justify-end ml-auto">
      <LightSwitch />
      <button class="btn !bg-transparent ml-3 p-0" use:popup={avatarPopup}>
        <Avatar src="/images/profile-picture-3.webp" width="w-10" />
      </button>
    </div>
    <div class="card p-4 shadow-xl" data-popup={avatarPopupId}>
      <nav class="list-nav text-sm text-gray-700 dark:text-white">
        <ul>
          <li>
            <div class="flex flex-col p-2">
              <div class="">Bonnie Green</div>
              <div class="truncate font-medium">name@wagewhiz.com</div>
            </div>
          </li>
          <hr class="!my-2" />
          <li><a href="/dashboard">Dashboard</a></li>
          <li><a href="/settings">Settings</a></li>
          <li><a href="/earnings">Earnings</a></li>
          <hr class="!my-2" />
          <li>
            <button class="w-full" on:click={() => $signOutMutation.mutate()}>Sign out</button>
          </li>
        </ul>
      </nav>
    </div>
  </div>
</nav>

<style lang="postcss">
  .navbar {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    z-index: 30;

    @apply bg-white dark:bg-surface-800;
    @apply border-b border-gray-200 dark:border-surface-500;
    @apply px-3 py-3 lg:px-5 lg:pl-3;
  }

  .navbar-inner {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    margin-left: auto;
    margin-right: auto;
  }
</style>
