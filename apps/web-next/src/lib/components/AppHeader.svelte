<script lang="ts">
  import { toggleMode } from "mode-watcher";
  import { Button } from "$lib/components/ui/button";
  import TheAvatar from "$lib/components/TheAvatar.svelte";
  import FluentAlert24Regular from "virtual:icons/fluent/alert-24-regular";
  import FluentWeatherSunny24Regular from "virtual:icons/fluent/weather-sunny-24-regular";
  import FluentWeatherMoon24Regular from "virtual:icons/fluent/weather-moon-24-regular";
</script>

<header class="h-14">
  <div class="container flex h-full max-w-full flex-row items-center">
    <Button href="/" variant="link" class="flex items-center">
      <img src="/assets/mage.svg" alt="Wagewhiz" height="24" width="24" class="mr-2 h-6 w-auto" />
      <span class="text-xl font-semibold tracking-tight">Wagewhiz</span>
    </Button>

    <div class="flex-1" />

    <div class="flex h-full flex-row items-center space-x-2">
      <Button variant="ghost" size="icon">
        <FluentAlert24Regular />
      </Button>

      <Button on:click={toggleMode} variant="ghost" size="icon">
        <FluentWeatherSunny24Regular
          class="rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
        />
        <FluentWeatherMoon24Regular
          class="absolute rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
        />
        <span class="sr-only">Change Theme</span>
      </Button>

      <div class="border-border h-3/5 border-r" />

      <div class="flex-center !ml-3.5">
        <TheAvatar />
      </div>
    </div>
  </div>
</header>
