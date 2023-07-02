<script lang="ts">
  import Avatar from "flowbite-svelte/Avatar.svelte";
  import Breadcrumb from "flowbite-svelte/Breadcrumb.svelte";
  import BreadcrumbItem from "flowbite-svelte/BreadcrumbItem.svelte";
  import Button from "flowbite-svelte/Button.svelte";
  import GradientButton from "flowbite-svelte/GradientButton.svelte";
  import Card from "flowbite-svelte/Card.svelte";
  import Label from "flowbite-svelte/Label.svelte";
  import Input from "flowbite-svelte/Input.svelte";
  import Toggle from "flowbite-svelte/Toggle.svelte";
  import CloudArrowUpIcon from "virtual:icons/ph/cloud-arrow-up";
  import FloppyDiskIcon from "virtual:icons/ph/floppy-disk";

  let usingPassword = false;

  const toggleUsingPassword = () => (usingPassword = !usingPassword);
</script>

<div class="grid grid-cols-1 px-4 pt-4 gap-4 dark:bg-gray-900">
  <div class="xl:mb-2">
    <Breadcrumb class="mb-5">
      <BreadcrumbItem href="/" home>Home</BreadcrumbItem>
      <BreadcrumbItem href="/people">People</BreadcrumbItem>
      <BreadcrumbItem>New Person</BreadcrumbItem>
    </Breadcrumb>
    <h1 class="text-xl font-semibold text-gray-900 sm:text-2xl dark:text-white">New Person</h1>
  </div>
  <Card class="!container w-full">
    <div class="items-center sm:flex 2xl:flex sm:space-x-4 2xl:space-x-4">
      <Avatar class="mb-4 w-28 h-28 sm:mb-0" size="lg" />
      <div>
        <h3 class="mb-1 text-xl font-bold text-gray-900 dark:text-white">Profile picture</h3>
        <div class="mb-4 text-sm text-gray-500 dark:text-gray-400">
          JPG, GIF or PNG. Max size of 800K
        </div>
        <div class="flex items-center space-x-2.5">
          <GradientButton size="sm">
            <CloudArrowUpIcon class="mr-2" height={20} width={20} />
            <span>Upload picture </span>
          </GradientButton>
          <Button size="sm" color="alternative">Delete</Button>
        </div>
      </div>
    </div>
  </Card>
  <Card class="!container w-full pb-6 sm:pb-8">
    <h3 class="mb-4 text-xl font-semibold dark:text-white">General information</h3>
    <div class="grid grid-cols-6 gap-6 max-w-5xl">
      <div class="col-span-6 sm:col-span-3">
        <Label for="first_name" class="mb-2">First name</Label>
        <Input type="text" name="first_name" id="first_name" placeholder="John" required />
      </div>
      <div class="col-span-6 sm:col-span-3">
        <Label for="last_name" class="mb-2">Last name</Label>
        <Input type="text" name="last_name" id="last_name" placeholder="Doe" required />
      </div>
      <div class="col-span-6 sm:col-span-3">
        <Label for="country" class="mb-2">Country</Label>
        <Input type="text" name="country" id="country" placeholder="United States" required />
      </div>
      <div class="col-span-6 sm:col-span-3">
        <Label for="city" class="mb-2">City</Label>
        <Input type="text" name="city" id="city" placeholder="e.g. San Francisco" required />
      </div>
      <div class="col-span-6 sm:col-span-3">
        <Label for="address" class="mb-2">Address</Label>
        <Input type="text" name="address" id="address" placeholder="e.g. California" required />
      </div>
      <div class="col-span-6 sm:col-span-3">
        <Label for="email" class="mb-2">Email</Label>
        <Input type="text" name="email" id="email" placeholder="example@company.com" required />
      </div>
      <div class="col-span-6 sm:col-span-3">
        <Label for="phone" class="mb-2">Phone Number</Label>
        <Input type="text" name="phone" id="phone" placeholder="e.g. +(12)3456 789" required />
      </div>
      <div class="col-span-6 sm:col-span-3">
        <Label for="birthday" class="mb-2">Birthday</Label>
        <Input type="text" name="birthday" id="birthday" placeholder="15/08/1990" required />
      </div>
      <div class="col-span-6 sm:col-span-3">
        <Label for="organization" class="mb-2">Organization</Label>
        <Input
          type="text"
          name="organization"
          id="organization"
          placeholder="Company Name"
          required
        />
      </div>
      <div class="col-span-6 sm:col-span-3">
        <Label for="role" class="mb-2">Role</Label>
        <Input type="text" name="role" id="role" placeholder="React Developer" required />
      </div>
      <div class="col-span-6 sm:col-span-3">
        <Label for="department" class="mb-2">Department</Label>
        <Input type="text" name="department" id="department" placeholder="Development" required />
      </div>
      <div class="col-span-6 sm:col-span-3">
        <Label for="zip_code" class="mb-2">Zip/postal code</Label>
        <Input type="text" name="zip_code" id="zip_code" placeholder="123456" required />
      </div>
    </div>
  </Card>
  <Card class="!container w-full">
    <div class="flex items-center">
      <button class="mr-2" on:click={toggleUsingPassword}>
        <h3 class="text-xl font-semibold dark:text-white">Password information</h3>
      </button>
      <Toggle size="small" bind:checked={usingPassword} />
    </div>
    {#if usingPassword}
      <div class="grid grid-cols-6 gap-6 mt-4 max-w-5xl">
        <div class="col-span-6 sm:col-span-3">
          <Label for="password" class="mb-2">Password</Label>
          <Input type="password" name="password" id="password" placeholder="********" required />
        </div>
        <div class="col-span-6 sm:col-span-3">
          <Label for="confirm_password" class="mb-2">Confirm Password</Label>
          <Input
            type="password"
            name="confirm_password"
            id="confirm_password"
            placeholder="********"
            required
          />
        </div>
      </div>
    {/if}
  </Card>
  <Card class="!container w-full">
    <div class="flex items-center space-x-2.5">
      <GradientButton size="sm" color="blue">
        <FloppyDiskIcon class="mr-2" height={20} width={20} />
        <span>Save</span>
      </GradientButton>
      <Button size="sm" color="alternative">Cancel</Button>
    </div>
  </Card>
</div>
