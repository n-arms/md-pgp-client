<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Store } from "@tauri-apps/plugin-store";
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { goto } from "$app/navigation";

  let fileName = $state("");
  
  onMount(async () => {
    const store = await Store.load("settings.json");
    const hasSetup = await store.get("hasSetup");
    if (!hasSetup) {
      goto("/setup");
    } else {
      await invoke("load_store", {});
    }
  });

  const createFile = async () => {
    console.log("Creating file", fileName);
    const uuid = await invoke<string>("create_document", { name: fileName });
    const params = new URLSearchParams();
    params.set('uuid', uuid);
    params.set('name', fileName);
    goto(`/file?${params.toString()}`);
  };
</script>

<div class="h-full w-full flex flex-col justify-center items-center">
  <h1 class="scroll-m-20 text-6xl font-extrabold tracking-tight lg:text-7xl">
    MD PGP
  </h1>
  <div class="flex flex-col gap-4 mt-8">
    <div class="flex flex-row gap-4">
      <Input placeholder="Enter file name" type="text" bind:value={fileName}/>
      <Button variant="outline" onclick={createFile}>Create File</Button>
    </div>
    <Button variant="outline" href="/setup">Setup</Button>
  </div>
</div>
