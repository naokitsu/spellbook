<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { listen } from '@tauri-apps/api/event'
  import {writable} from "svelte/store";
  import {onMount} from "svelte";


  const online = writable();
  onMount(async () => {
    let a = await invoke("client_state");
    if (typeof a === "string") {
      online.set(a);
    }
  });

  const unlisten = listen('LoLClientEvent', (event) => {
    if (typeof event.payload === "string") {
      online.set(event.payload);
    }
  })
</script>

<div>
  <p>{$online}</p>
</div>