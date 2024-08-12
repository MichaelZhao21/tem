<script lang="ts">
    import Tab from "./Tab.svelte";
</script>

<div class="tab-list">
    <Tab />
</div>

<style>
    .tab-list-container {
        flex-grow: 1;
        display: flex;
        flex-direction: column;
    }

    .tab-list {
        background-color: #282828;
        height: 40px;
        width: 100%;
        display: flex;
        flex-direction: row;
    }
</style>
