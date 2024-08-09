<script lang="ts">
    import './globals.css';
    import Sidebar from "../components/Sidebar.svelte";
    import Tabs from '../components/Tabs.svelte';
</script>

<div class="container">
    <Sidebar />
    <Tabs />
</div>

<style>
    .container {
        display: flex;
        flex-direction: row;
        height: 100%;
    }
</style>
