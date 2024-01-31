<script lang="ts">
  import Window from "./lib/Window.svelte";
  import Sidebar from "./lib/Sidebar.svelte";
  import SidebarIcon from "./lib/SidebarIcon.svelte";
  import FaBacon from 'svelte-icons/fa/FaBacon.svelte';
  import FaAppleAlt from 'svelte-icons/fa/FaAppleAlt.svelte';
  import Panel from "./lib/Panel.svelte";
  import Greet from "./lib/Greet.svelte";
  import Blank from "./lib/Blank.svelte";
  import CurrentRunes from "./lib/CurrentRunes.svelte";

  let selectedId = "blank";
</script>

<Window {selectedId}>
  <Sidebar>
    <SidebarIcon id="main" label="Main panel">
      <FaBacon  />
    </SidebarIcon>
    <SidebarIcon id="blank" label="Blank panel">
      <FaAppleAlt />
    </SidebarIcon>
    <SidebarIcon id="runes-current" label="Current runes">
      <FaAppleAlt />
    </SidebarIcon>
  </Sidebar>
  <Panel id="main">
    <Greet/>
  </Panel>
  <Panel id="blank">
    <Blank/>
  </Panel>
  <Panel id="runes-current">
    <CurrentRunes/>
  </Panel>
</Window>