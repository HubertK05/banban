<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let name = "";
  let greetMsg = "";

  async function greet() {
    greetMsg = await invoke("greet", { name });
  }
</script>

<div>
  <form on:submit|preventDefault={greet}>
    <input
      class="input p-2"
      id="greet-input"
      placeholder="Enter a name..."
      bind:value={name}
    />
    <button class="btn variant-filled" type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>
</div>
