<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import { invoke } from "@tauri-apps/api/core";

  let rust_msg = $state("");

  const beforeInputHandler = async e => {
    const {selectionStart, selectionEnd} = e.target;
    const args = { start: selectionStart, end: selectionEnd };
    if (e.inputType === "insertText") {
      args.text = e.data;
      await invoke("insert_text", args);
      rust_msg = await invoke("get_text", {});
    } else if (e.inputType === "deleteContentBackward") {
      await invoke("delete_text", args);
      rust_msg = await invoke("get_text", {});
    } else if (e.inputType === "insertLineBreak") {
      args.text = "\n";
      await invoke("insert_text", args);
    } else {
      alert(`Unknown input type ${e.inputType}`);
    }
  };
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
    onbeforeinput={beforeInputHandler}
    placeholder="Type your markdown content here..."
    class="w-full h-96 border border-gray-300 rounded-md p-2 mt-4"
  ></textarea>
  <p>{rust_msg}</p>
</div>
