<script lang="ts">
    import '../app.css';
    import Dock from "$lib/Dock.svelte";
    import favicon from '$lib/assets/favicon_small.png';
    import NavBar from "$lib/NavBar.svelte";
    import LoginRegister from "$lib/LoginRegister.svelte";

    // Authentication
    import { auth } from '$lib/stores/auth';
    $: loggedIn = $auth !== null;
</script>

<svelte:head>
    <link rel="icon" href={favicon}/>
</svelte:head>


{#if loggedIn}
    <NavBar/>
    <div class="m-2">
        <slot/>
    </div>
    <Dock/>
{:else}
    <LoginRegister/>
{/if}
