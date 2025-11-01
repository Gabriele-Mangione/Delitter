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
  <!-- keep the nav on top of Leaflet controls -->
  <header class="sticky top-0 z-[2000] bg-base-100">
    <NavBar />
  </header>

  <div class="m-2 mb-[64px]">
    <slot/>
  </div>

  <Dock/>
{:else}
  <LoginRegister/>
{/if}


<style>
    .spacer {
        height: 72px;
        width: 100%;
        background: transparent;
    }

    .main-content-area {
        /*height: calc(100% - (64px * 2) - 8px);*/
        /*max-height: calc(100% - (64px * 2) - 8px);*/
    }
</style>