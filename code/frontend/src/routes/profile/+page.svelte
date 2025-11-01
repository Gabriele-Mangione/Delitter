<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { auth, username as userStore } from '$lib/stores/auth';

    let username: string = 'Unknown';

    function decodeJwt(token: string | null) {
        try {
            if (!token) return {};
            const parts = token.split('.');
            if (parts.length < 2) return {};
            // base64url -> base64
            const payload = parts[1].replace(/-/g, '+').replace(/_/g, '/');
            const json = decodeURIComponent(atob(payload).split('').map(function(c) {
                return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
            }).join(''));
            return JSON.parse(json);
        } catch {
            return {};
        }
    }

    let unsubscribe: () => void;

    onMount(() => {
        // subscribe to username store so the UI updates when it changes
        unsubscribe = userStore.subscribe((v) => {
            if (v) username = v;
        });

        // if store has no username yet, fallback to decoding jwt (or localStorage)
        if (!username) {
            const token = (typeof window !== 'undefined' && localStorage.getItem('jwt')) ?? null;
            const payload: any = decodeJwt(token);
            username = payload?.username ?? payload?.sub ?? 'Unknown';
        }
    });

    // cleanup subscription if component unmounts
    // (Svelte will call this if you return a function from onMount, but keeping explicit)
    // @ts-ignore
    onDestroy?.(() => unsubscribe?.());

    function logout() {
        // clear both jwt and username
        // @ts-ignore
        auth.clear?.();
        // @ts-ignore
        userStore.clear?.();
    }
</script>

<div class="recording-page fixed inset-0 flex items-center justify-center overflow-hidden">
    <div class="card max-w-md w-full mx-auto bg-base-100 shadow-md p-6">
        <div class="card-body">
            <div class="flex flex-col items-center text-center gap-4">
                <!-- avatar placeholder (centered, larger) -->
                <div class="avatar">
                    <div class="w-20 h-20 rounded-full flex items-center justify-center bg-neutral text-neutral-content">
                        <!-- simple user icon placeholder -->
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5.121 17.804A9 9 0 0112 15a9 9 0 016.879 2.804M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
                        </svg>
                    </div>
                </div>

                <div class="mt-2 mb-4">
                    <span class="badge badge-lg">{username}</span>
                </div>

                <div class="flex">
                    <button class="btn btn-primary" on:click={logout}>Logout</button>
                </div>
            </div>
        </div>
    </div>
</div>
