<script>
  import CaretRightIcon from "~icons/ph/caret-right";
  import HouseIcon from "~icons/ph/house-fill";
</script>

<div class="grid grid-cols-1 px-4 pt-4 gap-4">
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
      <li>Edit Person Information</li>
    </ol>
  </div>
  <div class="xl:mb-2">
    <h1 class="h2">Edit Person Information</h1>
  </div>
</div>
