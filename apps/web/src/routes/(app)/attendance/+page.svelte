<script lang="ts">
  import * as Card from "$lib/components/ui/card";
</script>

<div>
  <div class="container">
    <Card.Root>
      <Card.Header>
        <Card.Title class="text-3xl font-semibold tracking-tight">
          Sure, time is money, but...
        </Card.Title>
        <!-- <Card.Description>
          ...the best things in life are often free. So take a break, you deserve it.
        </Card.Description> -->
        <Card.Description>
          <span
            >...your well-being is priceless, and it worries me when you push yourself too hard.</span
          >
          <br />
          <span>Please, take a break and allow yourself to relax.</span>
        </Card.Description>
      </Card.Header>
      <Card.Content>
        <p>Card Content</p>
      </Card.Content>
    </Card.Root>
  </div>
</div>
