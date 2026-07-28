<script lang="ts">
    import { onMount } from "svelte";

    let commandLineOpen = $state(false);

    const commandLineToggle = (event: KeyboardEvent) => {
        if (event.key !== ":") return;
        commandLineOpen = commandLineOpen === true ? false : true;
    };

    onMount(() => {
        addEventListener("keydown", commandLineToggle);
        return () => {
            removeEventListener("keydown", commandLineToggle);
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
