<script lang="ts">
  import '../app.css';
  import Dock from '$lib/Dock.svelte';
  import favicon from '$lib/assets/favicon_small.png';
  import NavBar from '$lib/NavBar.svelte';
  import LoginRegister from '$lib/LoginRegister.svelte';
  import { auth } from '$lib/stores/auth';
  $: loggedIn = $auth !== null;
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
</svelte:head>

{#if loggedIn}
  <!-- keep navbar above the map -->
  <header class="sticky top-0 z-[2000] bg-base-100">
    <NavBar />
  </header>

  <!-- reserve vertical space for the Dock so content (map) doesn't sit beneath it -->
  <div class="m-2 pb-[72px]">
    <slot />
  </div>

  <!-- Dock floats above everything -->
  <footer class="fixed bottom-0 inset-x-0 z-[3000]">
    <Dock />
  </footer>
{:else}
  <LoginRegister />
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