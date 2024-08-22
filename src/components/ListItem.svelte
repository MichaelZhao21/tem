<script lang="ts">
    import { activeDoc } from "../store";

    export let display = "[unloaded]";
    export let url = "unloaaded doc";

    const activateDoc = () => {
        $activeDoc = url;
    };
</script>

<button class="item" on:click={activateDoc}>{display}</button>

<style>
    .item {
        padding: 0.25rem 0;
        font-size: 0.75rem;
        text-decoration: none;
    }

    .item:hover {
        text-decoration: underline;
    }
</style>
