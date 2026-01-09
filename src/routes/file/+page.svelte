<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let message = $state("");
  let rust_msg = $state("");

  $effect(() => {
    const update_rust_msg = async () => {
      try {
        let result = await invoke("textarea_updated", { value: message });
        rust_msg = result
      } catch (err) {
        console.error("Failed to call Rust:", err);
      }
    };

    update_rust_msg()
  })
</script>

<main>
  <div class="header-bar">
    <h1 class="file-name">Insert file name here</h1>
    <input id="new-user" type="text" placeholder="User key">
    <button>Add User</button>
    <a href="/">Home</a>
  </div>
  <textarea rows=10 cols=10 bind:value={message}></textarea>
  <p>{rust_msg}</p>
</main>

<style>
.header-bar {
  margin: 0;
  display: flex;
  flex-direction: row;
}
</style>
