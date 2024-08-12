<script>
    import TagItem from "./TagItem.svelte";

</script>
<div class="sidebar">
    <h1 class="heading">tags</h1>
    <div class="tags-container">
        <TagItem />
    </div>
</div>

<style>
    .sidebar {
        height: calc(100% - 2rem);
        width: 300px;
        background-color: #282828;
        padding: 1rem;
    }

    .heading {
        font-size: 1.25rem;
        font-weight: 700;
        color: #575757;
        margin-bottom: 0.5rem;
    }

    .tags-container {
        display: flex;
        flex-direction: column;
        padding-left: 1rem;
    }
</style>
