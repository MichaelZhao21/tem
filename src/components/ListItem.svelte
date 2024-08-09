<script lang="ts">
    export let display = "[unloaded]";
    export let url = "";
</script>

<a href={url} class="item">{display}</a>

<style>
    .item {
        margin: 0.5rem 0;
    }
</style>
