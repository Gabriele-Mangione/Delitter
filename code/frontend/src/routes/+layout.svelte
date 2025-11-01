<script lang="ts">
    import '../app.css';
    import Dock from "$lib/Dock.svelte";
    import favicon from '$lib/assets/favicon.svg';
</script>

<svelte:head>
    <link rel="icon" href={favicon}/>
</svelte:head>

<slot />

<Dock></Dock>
