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

  const links = [
    { name: "Dashboard", href: "/dashboard", icon: FluentApps24Regular },
    { name: "People", href: "/people", icon: FluentPeopleCommunity24Regular },
    { name: "Time & Attendance", href: "/attendance", icon: FluentTimer24Regular },
    { name: "Payroll", href: "/payroll", icon: FluentMoneyCalculator24Regular },
    { name: "Recruitment", href: "/recruitment", icon: FluentPersonSearch24Regular },
    { name: "Insights", href: "/insights", icon: FluentCalendarDataBar24Regular },
    { name: "Settings", href: "/settings", icon: FluentSettings24Regular },
  ];
</script>

<div class="the-app-nav border-b pt-0.5">
  <div class="container flex max-w-full flex-row">
    <ul class="flex flex-row space-x-1">
      {#each links as link}
        <li class="flex flex-row pb-2">
          <Button
            href={link.href}
            variant="ghost"
            class={cn(
              "flex flex-1 flex-row items-center justify-start",
              $page.url.pathname === link.href && "bg-accent/60",
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

<style>
  .the-app-nav {
    overflow-x: auto;
    scrollbar-width: none;
  }
</style>
