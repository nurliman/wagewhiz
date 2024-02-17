<script lang="ts">
  import FluentWeatherSunny24Regular from "virtual:icons/fluent/weather-sunny-24-regular";
  import FluentWeatherMoon24Regular from "virtual:icons/fluent/weather-moon-24-regular";
  import { toggleMode } from "mode-watcher";
  import { Button } from "$lib/components/ui/button";
</script>

<Button
  on:click={toggleMode}
  variant="ghost"
  class="flex flex-1 flex-row items-center justify-start"
>
  <FluentWeatherSunny24Regular class="flex dark:hidden" />
  <FluentWeatherMoon24Regular class="hidden dark:flex" />
  <span class="ml-3.5">Change Theme</span>
</Button>
