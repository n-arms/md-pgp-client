<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Button } from "$lib/components/ui/button/index.js";
</script>

<main class="container">
  <h1>md pgp</h1>
  <Button variant="outline" href="/file">File</Button>
  <button
    type="button"
    class="border-2 border-gray-700"
    on:click={() => console.log("bello")}
  >
    Click Me!
  </button>
  <a href="/file">File</a>
  <a href="/setup">Setup</a>
</main>
