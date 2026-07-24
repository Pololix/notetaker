<script lang="ts">
    import { onMount } from "svelte";

    let commandLineOpen = $state(true);
    const callback = (event: KeyboardEvent) => {
        if (event.key !== ":") return;
        commandLineOpen = commandLineOpen === true ? false : true;
    };

    onMount(() => {
        addEventListener("keydown", callback);
        return () => {
            removeEventListener("keydown", callback);
        };
    });
</script>

<main>
    {#if commandLineOpen}
        <input class="cmdline" />
    {/if}
</main>

<style>
    .cmdline {
        position: fixed;
        left: 5px;
        right: 5px;
        bottom: 5px;
    }
</style>
