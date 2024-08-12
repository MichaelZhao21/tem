<script>
    import { activeDoc } from "../store";
    import ListItem from "./ListItem.svelte";

    export let name = "[undefined]";
    let expanded = false;

    // TODO: set expanded to true if path matches name

    const toggleList = () => {
        expanded = !expanded;
    };

    const activateDoc = () => {
        $activeDoc = name;
    };
</script>

<div class="container">
    <div class="line-item">
        <button on:click={toggleList} class="triangle-button">
            <svg
                width="8"
                height="11"
                viewBox="0 0 8 11"
                xmlns="http://www.w3.org/2000/svg"
                class="triangle"
            >
                <path d="M7.5 5.5L-3.98811e-09 11L0 -3.27835e-07L7.5 5.5Z" />
            </svg>
        </button>
        <button
            class="name"
            on:click={() => {
                toggleList();
                activateDoc();
            }}
        >
            {name}
        </button>
    </div>
    <div class={"inner-container " + (expanded ? "" : "hidden")}>
        <div class="line"></div>
        <div class="list">
            <ListItem />
            <ListItem />
            <ListItem />
        </div>
    </div>
</div>

<style>
    /* TODO: sort this mess */
    .line-item {
        display: flex;
        flex-direction: row;
        justify-content: flex-start;
    }

    .triangle-button {
        padding: 0 0.25rem;
    }

    .name {
        margin: 0.5rem 0;
        font-size: 1.25rem;
        color: #949494;
        text-decoration: none;
    }

    .name:hover {
        text-decoration: underline;
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

    .triangle {
        fill: #949494;
    }
</style>
