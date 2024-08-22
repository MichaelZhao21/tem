<script lang="ts">
    import "./globals.css";
    import Sidebar from "../components/Sidebar.svelte";
    import TabList from "../components/TabList.svelte";
    import Doc from "../components/Doc.svelte";
</script>

<div class="container">
    <Sidebar />
    <div class="right-container">
        <TabList />
        <Doc />
    </div>
</div>

<style>
    .container {
        display: flex;
        flex-direction: row;
        height: 100%;
        background-color: var(--background);
    }

    .right-container {
        display: flex;
        flex-grow: 1;
        flex-direction: column;
    }
</style>
