<script lang="ts">
    import '../app.css';
    import {page} from '$app/stores';
    import {resolve} from "$app/paths";
    import {auth} from "$lib/stores/auth";

    export function pathToTitle(path: string): string {
        const segment = path.replace(/^\/+/, ''); // remove leading slash
        if (!segment) return 'Home';
        return segment.charAt(0).toUpperCase() + segment.slice(1);
    }

    $: title = pathToTitle($page.url.pathname);

    function logout() {
        auth.clear();
    }
</script>

<div class="navbar bg-base-100">
    <div class="flex-1">
        <span class="text-2xl font-medium">{title}</span>
    </div>
    <div class="dropdown dropdown-end">
        <div tabindex="0" role="button" class="btn btn-circle avatar">
            <div class="w-10 h-10 rounded-full flex items-center justify-center">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                     stroke-width="1.5" stroke="currentColor" class="size-6">
                    <path stroke-linecap="round" stroke-linejoin="round"
                          d="M15.75 6a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0ZM4.501 20.118a7.5 7.5 0 0 1 14.998 0A17.933 17.933 0 0 1 12 21.75c-2.676 0-5.216-.584-7.499-1.632Z"/>
                </svg>
            </div>
        </div>
        <ul tabindex="-1"
            class="menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-52 p-2 shadow">
            <li><a href={resolve('/profile')}>Profile</a></li>
            <li>
                <button on:click={logout}>Logout</button>
            </li>
        </ul>
    </div>
</div>