<script lang="ts">
    import { getContext } from "svelte";
    import { tabContextId } from "./Window.svelte"

    export let id: string;

    const { selectedTab } = getContext(tabContextId);
</script>

{#if $selectedTab === id}
    <slot/>
{/if}