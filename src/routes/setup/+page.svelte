<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
</script>

<div class="container w-full mx-auto p-4">
  <h1 class="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
    Create Account / Log In
  </h1>
  <input
    id="pgp-key"
    class="w-full mt-4"
    type="text"
    placeholder="Enter PGP public key file path"
  />
  <div class="flex flex-row space-x-4 mt-4">
    <Button variant="outline">Submit</Button>
    <Button variant="outline" href="/">Home</Button>
  </div>
</div>
