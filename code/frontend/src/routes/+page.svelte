<script>
    import { goto } from '$app/navigation';
    import { onMount } from 'svelte';
    import {resolve} from "$app/paths";

    onMount(() => {
        goto(resolve('/recording'));
    });
</script>
