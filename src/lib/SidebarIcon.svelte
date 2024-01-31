<script lang="ts">
    import {getContext} from "svelte";
    import {tabContextId} from "./Window.svelte";
    export let label: string;
    export let id: string;


    function getMethods(obj)
    {
        var res = [];
        for(var m in obj) {
            if(typeof obj[m] == "function") {
                res.push(m)
            }
        }
        return res;
    }


    const { selectTab, selectedTab } = getContext(tabContextId);
</script>

<button class="sidebar-icon group" on:click={() => selectTab(id)} class:sidebar-icon-selected="{$selectedTab === id}">
    <slot />
    <span class="sidebar-tooltip group-hover:scale-100">
        {label}
    </span>
</button>


<style lang="postcss">
    .selected {
    }
</style>