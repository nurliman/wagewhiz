<script lang="ts">
  import Footer from "flowbite-svelte/Footer.svelte";
  import FooterLink from "flowbite-svelte/FooterLink.svelte";
  import FooterLinkGroup from "flowbite-svelte/FooterLinkGroup.svelte";
  import FooterCopyright from "flowbite-svelte/FooterCopyright.svelte";

  export let currentYear = new Date().getFullYear();
</script>

<Footer class="mt-4 shadow-none !bg-transparent">
  <FooterCopyright href="/" by="WageWhiz™" year={currentYear} />
  <FooterLinkGroup
    ulClass="flex flex-wrap items-center mt-3 text-sm text-gray-500 dark:text-gray-400 sm:mt-0"
  >
    <FooterLink href="/">About</FooterLink>
    <FooterLink href="/">Privacy Policy</FooterLink>
    <FooterLink href="/">Licensing</FooterLink>
    <FooterLink href="/">Contact</FooterLink>
  </FooterLinkGroup>
</Footer>
