<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import { invoke } from "@tauri-apps/api/core";

  let message = $state("");
  let rust_msg = $state("");

  $effect(() => {
    const update_rust_msg = async () => {
      try {
        const result = await invoke<string>("textarea_updated", { value: message });
        rust_msg = result;
      } catch (err) {
        console.error("Failed to call Rust:", err);
      }
    };

    update_rust_msg();
  });
</script>

<div class="container w-full mx-auto p-4">
  <div class="flex flex-col space-y-4">
    <h1 class="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
      Insert file name here
    </h1>
    <input class="border border-gray-300 rounded-md p-2" id="new-user" type="text" placeholder="User key" />
    <div class="flex flex-row space-x-4">
      <Button variant="outline">Add User</Button>
      <Button variant="outline" href="/">Home</Button>
    </div>
  </div>
  <textarea
    bind:value={message}
    placeholder="Type your markdown content here..."
    class="w-full h-96 border border-gray-300 rounded-md p-2 mt-4"
  ></textarea>
  <p>{rust_msg}</p>
</div>
