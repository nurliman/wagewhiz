<script lang="ts">
  import { page } from "$app/stores";
  import { cn } from "$lib/utils/shadcn";
  import { Button } from "$lib/components/ui/button";
  import FluentApps24Regular from "virtual:icons/fluent/apps-24-regular";
  import FluentPeopleCommunity24Regular from "virtual:icons/fluent/people-community-24-regular";
  import FluentTimer24Regular from "virtual:icons/fluent/timer-24-regular";
  import FluentMoneyCalculator24Regular from "virtual:icons/fluent/money-calculator-24-regular";
  import FluentPersonSearch24Regular from "virtual:icons/fluent/person-search-24-regular";
  import FluentCalendarDataBar24Regular from "virtual:icons/fluent/calendar-data-bar-24-regular";
  import FluentSettings24Regular from "virtual:icons/fluent/settings-24-regular";

  const navLinks = [
    { name: "Dashboard", href: "/dashboard", icon: FluentApps24Regular },
    { name: "People", href: "/people", icon: FluentPeopleCommunity24Regular },
    { name: "Time & Attendance", href: "/attendance", icon: FluentTimer24Regular },
    { name: "Payroll", href: "/payroll", icon: FluentMoneyCalculator24Regular },
    { name: "Recruitment", href: "/recruitment", icon: FluentPersonSearch24Regular },
    { name: "Insights", href: "/insights", icon: FluentCalendarDataBar24Regular },
    { name: "Settings", href: "/settings", icon: FluentSettings24Regular },
  ];

  let navContainer: HTMLDivElement;
</script>

<div bind:this={navContainer} class="the-app-nav">
  <div class="container flex max-w-full flex-row">
    <ul class="flex flex-row space-x-1">
      {#each navLinks as link}
        <li class="flex flex-row py-2">
          <Button
            href={link.href}
            variant="ghost"
            class={cn(
              "flex flex-1 flex-row items-center justify-start px-3 py-0.5",
              $page.url.pathname === link.href && "bg-accent/20 text-foreground",
            )}
          >
            <svelte:component this={link.icon} class="mr-2" />
            <span>{link.name}</span>
          </Button>
        </li>
      {/each}
    </ul>
  </div>
</div>

<style lang="postcss">
  .the-app-nav {
    @apply sticky top-0 z-10;
    @apply -mt-1.5 border-b;
    @apply overflow-x-auto;
    @apply bg-background/60;
    @apply backdrop-blur;

    scrollbar-width: none;

    :global(html:where(.dark)) & {
      @apply bg-background/80;
    }
  }
</style>
