<script lang="ts">
    import '../app.css';
    import {page} from '$app/stores';
    import {resolve} from "$app/paths";

    export function pathToTitle(path: string): string {
        const segment = path.replace(/^\/+/, ''); // remove leading slash
        if (!segment) return 'Home';
        return segment.charAt(0).toUpperCase() + segment.slice(1);
    }

    $: title = pathToTitle($page.url.pathname);
</script>

<div class="navbar bg-base-100">
    <div class="flex-1">
        <span class="text-2xl">{title}</span>
    </div>
    <div class="dropdown dropdown-end">
        <div tabindex="0" role="button" class="btn btn-ghost btn-circle avatar">
            <div class="w-10 rounded-full">
                <img
                        alt="Tailwind CSS Navbar component"
                        src="https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp"/>
            </div>
        </div>
        <ul tabindex="-1"
            class="menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-52 p-2 shadow">
            <li><a href={resolve('/profile')} >Profile</a></li>
            <li><a>Logout</a></li>
        </ul>
    </div>
</div>