<script lang="ts">
    import '../app.css';
    import Dock from "$lib/Dock.svelte";
    import favicon from '$lib/assets/favicon.svg';
    import NavBar from "$lib/NavBar.svelte";
</script>

<svelte:head>
    <link rel="icon" href={favicon}/>
</svelte:head>

<NavBar></NavBar>

<div class="m-2">
    <slot/>
</div>

<Dock></Dock>
