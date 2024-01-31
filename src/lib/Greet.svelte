<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { listen } from '@tauri-apps/api/event'
  import {writable} from "svelte/store";
  import {setContext} from "svelte";

  let auth_data: string = "";

  let online_str: string = "";

  const online = writable("None");

  const unsub = online.subscribe((value) => {
    online_str = value;
  })

  async function login_lol() {
    await invoke("login_lol", { path: "C:\\Riot Games\\League of Legends\\lockfile" })
  }


  const unlisten1 = listen('Online', (event) => {
    online.set("Online");
  })

  const unlisten2 = listen('Offline', (event) => {
    online.set("Offline");
  })

  async function get_auth() {
    auth_data = await invoke("get_auth")
  }

</script>

<div>

  <form class="row" on:submit|preventDefault={login_lol}>
    <button type="submit">Login</button>
  </form>

  <form class="row" on:submit|preventDefault={get_auth}>
    <button type="submit">Get_auth</button>
  </form>
  {auth_data}
  ---
  {online_str}

</div>