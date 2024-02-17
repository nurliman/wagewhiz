<script lang="ts">
  import { page } from "$app/stores";
  import { cn } from "$lib/utils/shadcn";
  import { Button } from "$lib/components/ui/button";
  import ThemeSwitcher from "$lib/components/ThemeSwitcher.svelte";
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

<aside class="border-border/40 flex w-72 flex-col border-r">
  <div class="mb-2.5 mt-3 flex h-16 items-center px-8">
    <span class="text-3xl font-semibold tracking-tight">Wagewhiz</span>
  </div>
  <nav class="flex flex-col">
    <ul class="flex flex-col space-y-1">
      {#each links as link}
        <li class="flex flex-row px-4">
          <Button
            href={link.href}
            variant="ghost"
            class={cn(
              "flex flex-1 flex-row items-center justify-start",
              $page.url.pathname === link.href && "bg-accent/60",
            )}
          >
            <svelte:component this={link.icon} class="mr-3.5" />
            <span>{link.name}</span>
          </Button>
        </li>
      {/each}
    </ul>
  </nav>

  <div class="flex-1" />

  <div class="flex p-4">
    <ThemeSwitcher />
  </div>
</aside>
