<script lang="ts">
    import { activeDoc } from "../store";

    export let name = "[unnamed]";

    // Update active if the state changes
    $: active = $activeDoc !== null && ($activeDoc as string) === name;

    // Close the document when clicked on
    function close() {
        console.log("CLICKED ON " + name);
    }
</script>

<div class={"tab-container " + (active ? "active" : "")}>
    <div>{name}</div>
    <div class="button-container">
        <button on:click={close} class="close">
            <!-- This is the "x" button svg -->
            <svg
                width="10"
                height="10"
                viewBox="0 0 10 10"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
                class="x"
            >
                <path d="M1 1L9 9M9 1L1 9" stroke-width="1.5" />
            </svg>
        </button>
    </div>
</div>

<style>
    .tab-container {
        padding: 0.5rem;
        padding-left: 1rem;
        display: flex;
        flex-direction: row;
        align-items: center;
        background-color: #303030;
    }

    .active {
        background-color: #353535;
    }

    .button-container {
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .close {
        width: 1rem;
        height: 1rem;
        padding: 0.25rem;
        margin: 0.25rem;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: 100ms;
    }

    .close:hover {
        background-color: #ffffff22;
    }

    .close:hover > .x {
        stroke: var(--primary);
    }

    .x {
        stroke: #e4e4e4;
        transition: 100ms;
    }
</style>
