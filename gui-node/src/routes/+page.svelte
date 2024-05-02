<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    let message = "";
    let done = "";

    async function handleMessage() {
        done = "Sending message...";
        done = await invoke("message_handler", { vx: 10.0, vy: 20.0 });
    }
</script>

<div>
    <input
        id="greet-input"
        placeholder="Enter a name..."
        bind:value={message}
    />
    <button on:click={handleMessage}>Send</button>
    <p>{done}</p>
</div>
