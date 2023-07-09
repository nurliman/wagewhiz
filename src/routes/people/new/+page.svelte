<script lang="ts">
  import DatePicker from "date-picker-svelte/DatePicker.svelte";
  import Avatar from "flowbite-svelte/Avatar.svelte";
  import Breadcrumb from "flowbite-svelte/Breadcrumb.svelte";
  import BreadcrumbItem from "flowbite-svelte/BreadcrumbItem.svelte";
  import Button from "flowbite-svelte/Button.svelte";
  import GradientButton from "flowbite-svelte/GradientButton.svelte";
  import Card from "flowbite-svelte/Card.svelte";
  import Helper from "flowbite-svelte/Helper.svelte";
  import Input from "flowbite-svelte/Input.svelte";
  import Label from "flowbite-svelte/Label.svelte";
  import Toggle from "flowbite-svelte/Toggle.svelte";
  import Textarea from "flowbite-svelte/Textarea.svelte";
  import CalendarIcon from "~icons/ph/calendar";
  import CloudArrowUpIcon from "~icons/ph/cloud-arrow-up";
  import FloppyDiskIcon from "~icons/ph/floppy-disk";
  import EyeIcon from "~icons/ph/eye";
  import EyeSlashIcon from "~icons/ph/eye-slash";
  import dayjs from "dayjs";
  import { superForm } from "sveltekit-superforms/client";
  import { Popover } from "svelte-smooth-popover";
  import { nanoid } from "$lib/utils/nanoid.ts";
  import { validateEmail } from "$lib/utils/validateEmail.ts";
  import type { PageData } from "./$types.ts";

  export let data: PageData;
  const fieldNameId = nanoid();
  const fieldNipId = nanoid();
  const fieldCountryId = nanoid();
  const fieldCityId = nanoid();
  const fieldAddressId = nanoid();
  const fieldZipCodeId = nanoid();
  const fieldEmailId = nanoid();
  const fieldPhoneId = nanoid();
  const fieldBirthdayId = nanoid();
  const fieldOrganizationId = nanoid();
  const fieldRoleId = nanoid();
  const fieldDepartmentId = nanoid();
  const fieldJoiningDateId = nanoid();
  const fieldPasswordId = nanoid();
  const fieldConfirmPasswordId = nanoid();
  let avatarInputFileElement: HTMLInputElement;
  let avatarImageSrc: string;
  let showBirthdayPopover = false;
  let showJoiningDatePopover = false;
  let usingPassword = false;
  let showPassword = false;
  let showConfirmPassword = false;

  const { form, errors, constraints } = superForm(data.form, {
    dataType: "json",
    validators: {
      email: (value) => (validateEmail(value) ? undefined : "Invalid email address"),
    },
  });

  $: displayBirthday = $form.birthday && dayjs($form.birthday).format("DD/MM/YYYY");
  $: displayJoiningDate = $form.joiningDate && dayjs($form.joiningDate).format("DD/MM/YYYY");

  const toggleBirthdayPopover = () => {
    showBirthdayPopover = !showBirthdayPopover;
  };

  const toggleJoiningDatePopover = () => {
    showJoiningDatePopover = !showJoiningDatePopover;
  };

  const toggleUsingPassword = () => {
    usingPassword = !usingPassword;
  };

  const handleAvatarChange = (event: Event) => {
    const target = event.target as HTMLInputElement;
    const file = target.files?.[0];
    if (file) {
      avatarImageSrc = URL.createObjectURL(file);
    }
  };

  const handleClearAvatar = () => {
    avatarImageSrc = "";
    if (avatarInputFileElement?.value) avatarInputFileElement.value = "";
  };
</script>

<form method="POST" class="grid grid-cols-1 px-4 pt-4 gap-4 dark:bg-gray-900">
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
      <Avatar src={avatarImageSrc} class="mb-4 w-28 h-28 sm:mb-0" size="lg" />
      <div>
        <h3 class="mb-1 text-xl font-bold text-gray-900 dark:text-white">Profile picture</h3>
        <div class="mb-4 text-sm text-gray-500 dark:text-gray-400">
          JPG, GIF or PNG. Max size of 800K
        </div>
        <div class="flex items-center space-x-2.5">
          <GradientButton
            size="sm"
            on:click={(e) => {
              e.preventDefault();
              avatarInputFileElement?.click?.();
            }}
          >
            <CloudArrowUpIcon class="mr-2" height={20} width={20} />
            <span>Upload picture</span>
          </GradientButton>
          <input
            bind:this={avatarInputFileElement}
            name="avatar"
            class="hidden"
            type="file"
            accept="image/*"
            on:change={handleAvatarChange}
          />
          <Button size="sm" color="alternative" on:click={handleClearAvatar}>Delete</Button>
        </div>
      </div>
    </div>
  </Card>
  <Card class="!container w-full pb-6 sm:pb-8">
    <h3 class="mb-4 text-xl font-semibold dark:text-white">General information</h3>
    <div class="grid grid-cols-6 gap-6 max-w-5xl">
      <div class="col-span-6 sm:col-span-3">
        <Label for={fieldNameId} class="mb-2">Full name</Label>
        <Input
          type="text"
          name="name"
          id={fieldNameId}
          placeholder="John Doe"
          aria-invalid={$errors.name ? "true" : undefined}
          bind:value={$form.name}
          {...$constraints.name}
        />
        {#if $errors.name}
          <Helper class="mt-2" color="red">{$errors.name}</Helper>
        {/if}
      </div>
      <div class="col-span-6 sm:col-span-3">
        <Label for={fieldNipId} class="mb-2">NIP</Label>
        <Input
          type="text"
          name="nip"
          id={fieldNipId}
          placeholder="541658798798549"
          aria-invalid={$errors.nip ? "true" : undefined}
          bind:value={$form.nip}
          {...$constraints.nip}
        />
      </div>
      <div class="col-span-6 sm:col-span-3">
        <Label for={fieldCountryId} class="mb-2">Country</Label>
        <Input
          type="text"
          name="country"
          id={fieldCountryId}
          placeholder="United States"
          aria-invalid={$errors.country ? "true" : undefined}
          bind:value={$form.country}
          {...$constraints.country}
        />
      </div>
      <div class="col-span-6 sm:col-span-3">
        <Label for={fieldCityId} class="mb-2">City</Label>
        <Input
          type="text"
          name="city"
          id={fieldCityId}
          placeholder="e.g. San Francisco"
          aria-invalid={$errors.city ? "true" : undefined}
          bind:value={$form.city}
          {...$constraints.city}
        />
      </div>
      <div class="col-span-6">
        <Label for={fieldAddressId} class="mb-2">Address</Label>
        <Textarea
          class="dark:bg-gray-600"
          name="address"
          id={fieldAddressId}
          placeholder="Jln. Jalan, no. 1"
          rows="4"
          aria-invalid={$errors.address ? "true" : undefined}
          bind:value={$form.address}
          {...$constraints.address}
        />
      </div>
      <div class="col-span-6 sm:col-span-3">
        <Label for={fieldZipCodeId} class="mb-2">Zip/postal code</Label>
        <Input
          type="text"
          name="zipCode"
          id={fieldZipCodeId}
          placeholder="123456"
          aria-invalid={$errors.zipCode ? "true" : undefined}
          bind:value={$form.zipCode}
          {...$constraints.zipCode}
        />
      </div>
      <div class="col-span-6 sm:col-span-3">
        <Label for={fieldEmailId} class="mb-2">Email</Label>
        <Input
          type="text"
          name="email"
          id={fieldEmailId}
          placeholder="example@company.com"
          aria-invalid={$errors.email ? "true" : undefined}
          color={$errors.email && "red"}
          bind:value={$form.email}
          {...$constraints.email}
        />
        {#if $errors.email}
          <Helper class="mt-2" color="red">{$errors.email}</Helper>
        {/if}
      </div>
      <div class="col-span-6 sm:col-span-3">
        <Label for={fieldPhoneId} class="mb-2">Phone Number</Label>
        <Input
          type="text"
          name="phone"
          id={fieldPhoneId}
          placeholder="e.g. 081234567890"
          aria-invalid={$errors.phone ? "true" : undefined}
          color={$errors.phone && "red"}
          bind:value={$form.phone}
          {...$constraints.phone}
        />
      </div>
      <div class="col-span-6 sm:col-span-3">
        <Label for={fieldBirthdayId} class="mb-2">Birthday</Label>
        <div>
          <Popover caretBg="" offset={10} open={showBirthdayPopover}>
            <DatePicker
              min={new Date(1900, 0, 1)}
              max={data.currentDate || new Date()}
              bind:value={$form.birthday}
              on:select={() => {
                setTimeout(() => (showBirthdayPopover = false), 100);
              }}
            />
          </Popover>
          <input type="hidden" name="birthday" value={$form.birthday} />
          <Input
            class="p-2.5"
            type="text"
            id={fieldBirthdayId}
            placeholder="17/08/1990"
            readonly
            value={displayBirthday}
            aria-invalid={$errors.birthday ? "true" : undefined}
            color={$errors.birthday && "red"}
            on:click={toggleBirthdayPopover}
          >
            <Button
              class="!p-1.5 focus:ring-2 focus:ring-gray-400"
              slot="right"
              color="none"
              on:click={toggleBirthdayPopover}
            >
              <CalendarIcon width={20} height={20} />
            </Button>
          </Input>
        </div>
      </div>
      <div class="col-span-6 sm:col-span-3">
        <Label for={fieldOrganizationId} class="mb-2">Organization</Label>
        <Input
          type="text"
          name="organization"
          id={fieldOrganizationId}
          placeholder="Company Name"
          aria-invalid={$errors.organization ? "true" : undefined}
          color={$errors.organization && "red"}
          bind:value={$form.organization}
          {...$constraints.organization}
        />
      </div>
      <div class="col-span-6 sm:col-span-3">
        <Label for={fieldRoleId} class="mb-2">Role</Label>
        <Input
          type="text"
          name="role"
          id={fieldRoleId}
          placeholder="React Developer"
          aria-invalid={$errors.role ? "true" : undefined}
          color={$errors.role && "red"}
          bind:value={$form.role}
          {...$constraints.role}
        />
      </div>
      <div class="col-span-6 sm:col-span-3">
        <Label for={fieldDepartmentId} class="mb-2">Department</Label>
        <Input
          type="text"
          name="department"
          id={fieldDepartmentId}
          placeholder="Development"
          aria-invalid={$errors.department ? "true" : undefined}
          color={$errors.department && "red"}
          bind:value={$form.department}
          {...$constraints.department}
        />
      </div>
      <div class="col-span-6 sm:col-span-3 relative">
        <Label for={fieldJoiningDateId} class="mb-2">Joining Date</Label>
        <div>
          <Popover caretBg="" offset={10} open={showJoiningDatePopover}>
            <DatePicker
              min={new Date(1900, 0, 1)}
              max={data.currentDate || new Date()}
              bind:value={$form.joiningDate}
              on:select={() => {
                setTimeout(() => (showJoiningDatePopover = false), 100);
              }}
            />
          </Popover>
          <input type="hidden" name="joiningDate" value={$form.joiningDate} />
          <Input
            class="p-2.5"
            type="text"
            id={fieldJoiningDateId}
            placeholder="15/08/1990"
            readonly
            value={displayJoiningDate}
            aria-invalid={$errors.joiningDate ? "true" : undefined}
            color={$errors.joiningDate && "red"}
            on:click={toggleJoiningDatePopover}
          >
            <Button
              id={fieldJoiningDateId}
              class="!p-1.5 focus:ring-2 focus:ring-gray-400"
              slot="right"
              color="none"
              on:click={toggleJoiningDatePopover}
            >
              <CalendarIcon width={20} height={20} />
            </Button>
          </Input>
        </div>
      </div>
    </div>
  </Card>
  <Card class="!container w-full">
    <div class="flex items-center">
      <button type="button" class="mr-2" on:click={toggleUsingPassword}>
        <h3 class="text-xl font-semibold dark:text-white">Password information</h3>
      </button>
      <Toggle size="small" bind:checked={usingPassword} />
    </div>
    {#if usingPassword}
      <div class="grid grid-cols-6 gap-6 mt-4 max-w-5xl">
        <div class="col-span-6 sm:col-span-3">
          <Label for={fieldPasswordId} class="mb-2">Password</Label>
          <Input
            class="p-2.5"
            type={showPassword ? "text" : "password"}
            name="password"
            id={fieldPasswordId}
            placeholder="********"
          >
            <Button
              class="!p-1.5 focus:ring-2 focus:ring-gray-400"
              slot="right"
              color="none"
              on:click={() => (showPassword = !showPassword)}
            >
              {#if showPassword}
                <EyeIcon width={20} height={20} />
              {:else}
                <EyeSlashIcon width={20} height={20} />
              {/if}
            </Button>
          </Input>
        </div>
        <div class="col-span-6 sm:col-span-3">
          <Label for={fieldConfirmPasswordId} class="mb-2">Confirm Password</Label>
          <Input
            class="p-2.5"
            type={showConfirmPassword ? "text" : "password"}
            name="confirm_password"
            id={fieldConfirmPasswordId}
            placeholder="********"
          >
            <Button
              class="!p-1.5 focus:ring-2 focus:ring-gray-400"
              slot="right"
              color="none"
              on:click={() => (showConfirmPassword = !showConfirmPassword)}
            >
              {#if showConfirmPassword}
                <EyeIcon width={20} height={20} />
              {:else}
                <EyeSlashIcon width={20} height={20} />
              {/if}
            </Button>
          </Input>
        </div>
      </div>
    {/if}
  </Card>
  <Card class="!container w-full">
    <div class="flex items-center space-x-2.5">
      <GradientButton size="sm" color="blue" type="submit">
        <FloppyDiskIcon class="mr-2" height={20} width={20} />
        <span>Save</span>
      </GradientButton>
      <Button size="sm" color="alternative">Cancel</Button>
    </div>
  </Card>
</form>
