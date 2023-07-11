<script lang="ts">
  export let currentYear = new Date().getFullYear();
</script>

<footer class="p-4 md:flex md:items-center md:justify-between md:p-6 mt-4">
  <span class="block text-sm text-gray-500 sm:text-center dark:text-gray-400">
    © {currentYear} <a href="/" class="hover:underline">WageWhiz™</a> All Rights Reserved.
  </span>
  <ul class="flex flex-wrap items-center mt-3 text-sm text-gray-500 dark:text-gray-400 sm:mt-0">
    <li class="mr-4 last:mr-0 md:mr-6"><a href="/" class="hover:underline">About</a></li>
    <li class="mr-4 last:mr-0 md:mr-6"><a href="/" class="hover:underline">Privacy Policy</a></li>
    <li class="mr-4 last:mr-0 md:mr-6"><a href="/" class="hover:underline">Licensing</a></li>
    <li class="mr-4 last:mr-0 md:mr-6"><a href="/" class="hover:underline">Contact</a></li>
  </ul>
</footer>
