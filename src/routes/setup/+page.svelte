<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { toast } from "svelte-sonner";
  import { Store } from "@tauri-apps/plugin-store";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from '@tauri-apps/plugin-dialog';
  
  import Home from "@lucide/svelte/icons/home";
  import Save from "@lucide/svelte/icons/save";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";

  let keyPath = $state("");
  let serverAddress = $state("");
  let firstSetup = $state(true);

  onMount(async () => {
    const store = await Store.load("settings.json");
    const storedAddressValue = (await store.get<string>("serverAddress")) || "";
    const storedHasSetup = (await store.get<boolean>("hasSetup")) || false;
    if (storedAddressValue) {
      serverAddress = storedAddressValue;
    }
    firstSetup = storedHasSetup;
  });

  const saveSettings = () => {
    Store.load("settings.json").then(async (store) => {
      await store.set("serverAddress", serverAddress);
      await store.set("keyPath", keyPath);
      await store.set("hasSetup", true);
      await store.save();
      await invoke("load_store", {});
      await invoke("create_account", {});
      goto("/");
      toast("Server address saved!");
    });
  };

  const openKeyFile = async () => {
    const selectedPath = await open({
      multiple: false,
      directory: false
    });

    if (selectedPath === null) {
      console.log("User cancelled the picker");
      return;
    }

    keyPath = selectedPath;
  };
</script>

<div class="container w-full mx-auto p-4">
  <h1 class="scroll-m-20 text-4xl tracking-tight lg:text-5xl">Setup</h1>
  <h3 class="scroll-m-20 text-xl tracking-tight lg:text-2xl">
    Create Account / Log In
  </h3>
  <Button variant="outline" onclick={openKeyFile}>Enter PGP private key file path</Button>
  <div class="flex flex-row space-x-4 mt-4"></div>
  <h3 class="scroll-m-20 text-xl tracking-tight lg:text-2xl">
    Link your server address
  </h3>
  <Input
    class="w-full mt-4"
    type="text"
    placeholder="Enter server address"
    bind:value={serverAddress}
  />
  <div class="flex flex-row space-x-4 mt-4">
    {#if firstSetup}
      <Button variant="outline" href="/"><Home />Home</Button>
    {/if}
    <Button variant="outline" onclick={saveSettings}><Save />Save</Button>
  </div>
</div>
