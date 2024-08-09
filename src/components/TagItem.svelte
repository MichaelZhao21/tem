<script>
    import ListItem from "./ListItem.svelte";

    export let name = "[undefined]";
    let expanded = false;
</script>

<div class="container">
    <button on:click={() => (expanded = !expanded)} class="name">{name}</button>
    <div class={"inner-container " + (expanded ? "" : "hidden")}>
        <div class="line"></div>
        <div class="list">
            <ListItem />
        </div>
    </div>
</div>

<style>
    .name {
        margin: 0.5rem 0;
        font-size: 1.25rem;
        color: #949494;
    }

    .inner-container {
        display: flex;
        flex-direction: row;
    }

    .hidden {
        display: none;
    }

    .line {
        width: 2px;
        margin: 0 0.5rem;
        background-color: #3c3c3c;
    }

    .list {
        display: flex;
        flex-direction: column;
        padding-left: 0.5rem;
    }
</style>
