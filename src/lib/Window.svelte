<script context="module" lang="ts">
    export const tabContextId = new Set();
</script>

<script lang="ts">
    import { setContext } from "svelte";
    import { writable } from "svelte/store";

    export let selectedId: string;

    const selectedTab = writable(selectedId);

    $: selectedTab.set(selectedId);

    setContext(tabContextId, {
        selectTab: (tabId: string) => {
            selectedId = tabId;
        },
        selectedTab,
    })
</script>

<div class="flex flex-row">
    <slot />
</div>
