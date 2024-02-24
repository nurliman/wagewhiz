<script lang="ts">
  import { onMount } from "svelte";
  import isAbsoluteUrl from "is-absolute-url";

  const LINKS = [
    { name: "About", href: "#" },
    { name: "Privacy", href: "#" },
    { name: "Docs", href: "#" },
    { name: "Contact", href: "#" },
    { name: "Github", href: "https://github.com/nurliman/wagewhiz" },
  ];

  let currentYear = new Date().getFullYear();

  onMount(() => {
    currentYear = new Date().getFullYear();
  });
</script>

<footer class="text-muted-foreground p-4 text-center text-sm leading-loose md:p-6">
  <div class="container flex flex-col-reverse items-center md:flex-row md:justify-between">
    <div class="md:text-left">
      © {currentYear} <a href="/" class="hover:text-foreground underline">Wagewhiz</a>.
    </div>

    <ul class="flex flex-row flex-wrap justify-center space-x-4 space-x-6">
      {#each LINKS as link}
        <li>
          <a
            href={link.href}
            target={isAbsoluteUrl(link.href) ? "_blank" : undefined}
            class="hover:text-foreground hover:underline">{link.name}</a
          >
        </li>
      {/each}
    </ul>
  </div>
</footer>
