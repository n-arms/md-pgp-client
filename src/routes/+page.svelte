<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Store } from "@tauri-apps/plugin-store";
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui/button/index.js";
  import { goto } from "$app/navigation";

  let fileList = $state([]);

  onMount(async () => {
    const store = await Store.load("settings.json");
    const hasSetup = await store.get("hasSetup");
    if (!hasSetup) {
      goto("/setup");
    } else {
      await invoke("load_store", {});
      fileList = invoke("get_file_list", {});
    }
  });
</script>

<div class="h-full w-full flex flex-col justify-center items-center">
  <h1 class="scroll-m-20 text-6xl font-extrabold tracking-tight lg:text-7xl">
    MD PGP
  </h1>
  <div class="flex flex-row space-x-4 mt-8">
    <Button variant="outline" href="/file">File</Button>
    <Button variant="outline" href="/setup">Setup</Button>
  </div>
</div>
