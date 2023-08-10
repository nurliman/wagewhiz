<script lang="ts">
  import getInitials from "initials";
  import DatePicker from "date-picker-svelte/DatePicker.svelte";
  import Portal from "$lib/components/Portal.svelte";
  import CalendarIcon from "~icons/ph/calendar";
  import CaretRightIcon from "~icons/ph/caret-right";
  import CloudArrowUpIcon from "~icons/ph/cloud-arrow-up";
  import FloppyDiskIcon from "~icons/ph/floppy-disk";
  import EyeIcon from "~icons/ph/eye";
  import EyeSlashIcon from "~icons/ph/eye-slash";
  import HouseIcon from "~icons/ph/house-fill";
  import dayjs from "dayjs";
  import { Avatar, SlideToggle, popup } from "@skeletonlabs/skeleton";
  import { superForm } from "sveltekit-superforms/client";
  import { nanoid } from "$lib/utils/nanoid.ts";
  import { validateEmail } from "$lib/utils/validateEmail.ts";
  import type { PopupSettings } from "@skeletonlabs/skeleton";
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
  let usingPassword = false;
  let showPassword = false;
  let showConfirmPassword = false;

  const { form, errors, constraints } = superForm(data.form, {
    dataType: "json",
    validators: {
      email: (value) => (validateEmail(value) ? undefined : "Invalid email address"),
    },
  });

  const birthdayPopupId = nanoid();
  const birthdayPopup: PopupSettings = {
    event: "focus-click",
    target: birthdayPopupId,
    closeQuery: ".date-time-picker .cell",
  };

  const joiningDatePopupId = nanoid();
  const joiningDatePopup: PopupSettings = {
    event: "focus-click",
    target: joiningDatePopupId,
    closeQuery: ".date-time-picker .cell",
  };

  $: displayBirthday = $form.birthday && dayjs($form.birthday).format("DD/MM/YYYY");
  $: displayJoiningDate = $form.joiningDate && dayjs($form.joiningDate).format("DD/MM/YYYY");
  $: initials = $form.name ? getInitials($form.name) : undefined;

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

<form method="POST" class="grid grid-cols-1 px-4 pt-4 gap-4">
  <div class="xl:mb-2">
    <ol class="breadcrumb text-sm space-x-1">
      <li class="crumb">
        <a href="/" class="btn btn-sm variant-soft">
          <HouseIcon height={14} width={14} />
          <span>Home</span>
        </a>
      </li>
      <li class="crumb-separator" aria-hidden><CaretRightIcon height={14} width={14} /></li>
      <li class="crumb"><a href="/people" class="btn btn-sm variant-soft">People</a></li>
      <li class="crumb-separator" aria-hidden><CaretRightIcon height={14} width={14} /></li>
      <li>New Person</li>
    </ol>
  </div>
  <div class="xl:mb-2">
    <h1 class="h2">New Person</h1>
  </div>
  <div class="card container w-full p-4 sm:p-6">
    <div class="items-center sm:flex 2xl:flex sm:space-x-4 2xl:space-x-4">
      <Avatar src={avatarImageSrc} {initials} class="mb-4 sm:mb-0" width="w-28" />
      <div>
        <h2 class="h4 mb-1">Profile picture</h2>
        <div class="mb-4 text-sm text-gray-500 dark:text-gray-400">
          JPG, GIF or PNG. Max size of 800K
        </div>
        <div class="flex items-center space-x-2.5">
          <button
            type="button"
            class="btn btn-sm bg-gradient-to-br variant-gradient-primary-secondary"
            on:click={(e) => {
              e.preventDefault();
              avatarInputFileElement?.click?.();
            }}
          >
            <CloudArrowUpIcon height={20} width={20} />
            <span>Upload picture</span>
          </button>
          <input
            bind:this={avatarInputFileElement}
            name="avatar"
            class="hidden"
            type="file"
            accept="image/*"
            on:change={handleAvatarChange}
          />
          <button type="button" class="btn btn-sm variant-ghost" on:click={handleClearAvatar}>
            Delete
          </button>
        </div>
      </div>
    </div>
  </div>
  <div class="card container w-full p-4 sm:p-6 pb-6 sm:pb-8">
    <h2 class="h3 mb-4">General information</h2>
    <div class="grid grid-cols-6 gap-6 max-w-5xl">
      <div class="col-span-6 sm:col-span-3 space-y-2">
        <label for={fieldNameId} class="label">Full name</label>
        <input
          id={fieldNameId}
          name="name"
          type="text"
          class="input"
          class:input-error={$errors.name}
          placeholder="John Doe"
          aria-invalid={$errors.name ? "true" : undefined}
          bind:value={$form.name}
          {...$constraints.name}
        />
        {#if $errors.name}
          <p class="text-xs text-error-500">{$errors.name}</p>
        {/if}
      </div>
      <div class="col-span-6 sm:col-span-3 space-y-2">
        <label for={fieldNipId} class="label">NIP</label>
        <input
          id={fieldNipId}
          name="nip"
          type="text"
          class="input"
          class:input-error={$errors.nip}
          placeholder="541658798798549"
          aria-invalid={$errors.nip ? "true" : undefined}
          bind:value={$form.nip}
          {...$constraints.nip}
        />
        {#if $errors.nip}
          <p class="text-xs text-error-500">{$errors.nip}</p>
        {/if}
      </div>
      <div class="col-span-6 sm:col-span-3 space-y-2">
        <label for={fieldCountryId} class="label">Country</label>
        <input
          id={fieldCountryId}
          type="text"
          name="country"
          class="input"
          class:input-error={$errors.country}
          placeholder="United States"
          aria-invalid={$errors.country ? "true" : undefined}
          bind:value={$form.country}
          {...$constraints.country}
        />
        {#if $errors.country}
          <p class="text-xs text-error-500">{$errors.country}</p>
        {/if}
      </div>
      <div class="col-span-6 sm:col-span-3 space-y-2">
        <label for={fieldCityId} class="label">City</label>
        <input
          id={fieldCityId}
          type="text"
          name="city"
          class="input"
          class:input-error={$errors.city}
          placeholder="e.g. San Francisco"
          aria-invalid={$errors.city ? "true" : undefined}
          bind:value={$form.city}
          {...$constraints.city}
        />
        {#if $errors.city}
          <p class="text-xs text-error-500">{$errors.city}</p>
        {/if}
      </div>
      <div class="col-span-6 space-y-2">
        <label for={fieldAddressId} class="label">Address</label>
        <textarea
          id={fieldAddressId}
          name="address"
          class="textarea"
          class:input-error={$errors.address}
          placeholder="Jln. Jalan, no. 1"
          rows="4"
          aria-invalid={$errors.address ? "true" : undefined}
          bind:value={$form.address}
          {...$constraints.address}
        />
        {#if $errors.address}
          <p class="text-xs text-error-500">{$errors.address}</p>
        {/if}
      </div>
      <div class="col-span-6 sm:col-span-3 space-y-2">
        <label for={fieldZipCodeId} class="label">Zip/postal code</label>
        <input
          id={fieldZipCodeId}
          type="text"
          name="zipCode"
          class="input"
          class:input-error={$errors.zipCode}
          placeholder="123456"
          aria-invalid={$errors.zipCode ? "true" : undefined}
          bind:value={$form.zipCode}
          {...$constraints.zipCode}
        />
        {#if $errors.zipCode}
          <p class="text-xs text-error-500">{$errors.zipCode}</p>
        {/if}
      </div>
      <div class="col-span-6 sm:col-span-3 space-y-2">
        <label for={fieldEmailId} class="label">Email</label>
        <input
          id={fieldEmailId}
          name="email"
          type="text"
          class="input"
          class:input-error={$errors.email}
          placeholder="example@company.com"
          aria-invalid={$errors.email ? "true" : undefined}
          bind:value={$form.email}
          {...$constraints.email}
        />
        {#if $errors.email}
          <p class="text-xs text-error-500">{$errors.email}</p>
        {/if}
      </div>
      <div class="col-span-6 sm:col-span-3 space-y-2">
        <label for={fieldPhoneId} class="label">Phone Number</label>
        <input
          id={fieldPhoneId}
          type="text"
          name="phone"
          class="input"
          class:input-error={$errors.phone}
          placeholder="e.g. 081234567890"
          aria-invalid={$errors.phone ? "true" : undefined}
          bind:value={$form.phone}
          {...$constraints.phone}
        />
        {#if $errors.phone}
          <p class="text-xs text-error-500">{$errors.phone}</p>
        {/if}
      </div>
      <div class="col-span-6 sm:col-span-3 space-y-2">
        <label for={fieldBirthdayId} class="label">Birthday</label>
        <div>
          <Portal target="body">
            <div class="shadow-xl" data-popup={birthdayPopupId}>
              <DatePicker
                min={new Date(1900, 0, 1)}
                max={data.currentDate || new Date()}
                bind:value={$form.birthday}
              />
            </div>
          </Portal>
          <input type="hidden" name="birthday" value={$form.birthday} />
          <div
            class="input-group input-group-divider grid-cols-[1fr_auto]"
            class:input-error={$errors.birthday}
            use:popup={birthdayPopup}
          >
            <input
              id={fieldBirthdayId}
              type="text"
              class="cursor-pointer"
              placeholder="17/08/1990"
              readonly
              value={displayBirthday}
              aria-invalid={$errors.birthday ? "true" : undefined}
            />
            <button type="button" class="variant-soft">
              <CalendarIcon />
            </button>
          </div>
        </div>
        {#if $errors.birthday}
          <p class="text-xs text-error-500">{$errors.birthday}</p>
        {/if}
      </div>
      <div class="col-span-6 sm:col-span-3 space-y-2">
        <label for={fieldOrganizationId} class="label">Organization</label>
        <input
          id={fieldOrganizationId}
          type="text"
          name="organization"
          class="input"
          class:input-error={$errors.organization}
          placeholder="Company Name"
          aria-invalid={$errors.organization ? "true" : undefined}
          bind:value={$form.organization}
          {...$constraints.organization}
        />
        {#if $errors.organization}
          <p class="text-xs text-error-500">{$errors.organization}</p>
        {/if}
      </div>
      <div class="col-span-6 sm:col-span-3 space-y-2">
        <label for={fieldRoleId} class="label">Role</label>
        <input
          id={fieldRoleId}
          type="text"
          name="role"
          class="input"
          class:input-error={$errors.role}
          placeholder="React Developer"
          aria-invalid={$errors.role ? "true" : undefined}
          bind:value={$form.role}
          {...$constraints.role}
        />
        {#if $errors.role}
          <p class="text-xs text-error-500">{$errors.role}</p>
        {/if}
      </div>
      <div class="col-span-6 sm:col-span-3 space-y-2">
        <label for={fieldDepartmentId} class="label">Department</label>
        <input
          id={fieldDepartmentId}
          type="text"
          name="department"
          class="input"
          class:input-error={$errors.department}
          placeholder="Development"
          aria-invalid={$errors.department ? "true" : undefined}
          bind:value={$form.department}
          {...$constraints.department}
        />
        {#if $errors.department}
          <p class="text-xs text-error-500">{$errors.department}</p>
        {/if}
      </div>
      <div class="col-span-6 sm:col-span-3 space-y-2">
        <label for={fieldJoiningDateId} class="label">Joining Date</label>
        <div>
          <Portal target="body">
            <div class="shadow-xl" data-popup={joiningDatePopupId}>
              <DatePicker
                min={new Date(1900, 0, 1)}
                max={data.currentDate || new Date()}
                bind:value={$form.joiningDate}
              />
            </div>
          </Portal>
          <input type="hidden" name="joiningDate" value={$form.joiningDate} />
          <div
            class="input-group input-group-divider grid-cols-[1fr_auto]"
            class:input-error={$errors.joiningDate}
            use:popup={joiningDatePopup}
          >
            <input
              id={fieldJoiningDateId}
              type="text"
              class="cursor-pointer"
              placeholder="15/08/1990"
              readonly
              value={displayJoiningDate}
              aria-invalid={$errors.joiningDate ? "true" : undefined}
            />
            <button type="button" class="variant-soft">
              <CalendarIcon />
            </button>
          </div>
        </div>
        {#if $errors.joiningDate}
          <p class="text-xs text-error-500">{$errors.joiningDate}</p>
        {/if}
      </div>
    </div>
  </div>
  <div class="card container w-full p-4 sm:p-6">
    <div class="flex items-center">
      <button type="button" class="mr-2" on:click={toggleUsingPassword}>
        <h2 class="h3">Password information</h2>
      </button>
      <SlideToggle
        name="usingPassword"
        active="bg-secondary-500"
        size="sm"
        bind:checked={usingPassword}
      />
    </div>
    {#if usingPassword}
      <div class="grid grid-cols-6 gap-6 mt-4 max-w-5xl">
        <div class="col-span-6 sm:col-span-3 space-y-2">
          <label for={fieldPasswordId} class="label">Password</label>
          <div class="input-group input-group-divider grid-cols-[1fr_auto]">
            <input
              type={showPassword ? "text" : "password"}
              name="password"
              id={fieldPasswordId}
              placeholder="********"
            />
            <button
              type="button"
              class="variant-soft"
              on:click={() => (showPassword = !showPassword)}
            >
              {#if showPassword}
                <EyeIcon width={20} height={20} />
              {:else}
                <EyeSlashIcon width={20} height={20} />
              {/if}
            </button>
          </div>
        </div>
        <div class="col-span-6 sm:col-span-3 space-y-2">
          <label for={fieldConfirmPasswordId} class="label">Confirm Password</label>
          <div class="input-group input-group-divider grid-cols-[1fr_auto]">
            <input
              type={showConfirmPassword ? "text" : "password"}
              name="confirm_password"
              id={fieldConfirmPasswordId}
              placeholder="********"
            />
            <button
              type="button"
              class="variant-soft"
              on:click={() => (showConfirmPassword = !showConfirmPassword)}
            >
              {#if showConfirmPassword}
                <EyeIcon width={20} height={20} />
              {:else}
                <EyeSlashIcon width={20} height={20} />
              {/if}
            </button>
          </div>
        </div>
      </div>
    {/if}
  </div>
  <div class="card container w-full p-4 sm:p-6">
    <div class="flex items-center space-x-2.5">
      <button type="submit" class="btn bg-gradient-to-br variant-gradient-primary-secondary">
        <FloppyDiskIcon height={20} width={20} />
        <span>Save</span>
      </button>
      <button type="button" class="btn variant-ghost">Cancel</button>
    </div>
  </div>
</form>
