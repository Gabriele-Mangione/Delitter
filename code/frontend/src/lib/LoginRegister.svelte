<script lang="ts">
    import { auth } from '$lib/stores/auth';
    import { PUBLIC_BACKEND_URL } from '$env/static/public';

    let username = '';
    let password = '';
    let response = '';
    let loading = false;
    let isSignup = true;
    const base = PUBLIC_BACKEND_URL;

    async function register() {
        response = '';
        loading = true;
        try {
            const api_route = isSignup ? "signup" : "signin";
            const api_url = `${base}/public/auth/${api_route}`
            const res = await fetch(api_url, {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify({username, password}),
            });

            const data = await res.json().catch(() => ({}));

            if (!res.ok) {
                response = data?.message ?? `Error ${res.status}`;
                return;
            }

            if (data.jwt) {
                auth.setToken(data.jwt); // triggers reactive update in layout
                response = 'Registered successfully. Token saved locally.';
            } else {
                response = 'Unexpected response format.';
            }
        } catch (err) {
            response = 'Network error';
        } finally {
            loading = false;
        }
    }

    function toggleSignup(){
        isSignup = !isSignup
    }

</script>

<div>
    <div class="flex flex-col gap-6 min-h-screen items-center justify-center bg-base-200">
        <form class="card w-96 bg-base-100 shadow-xl p-6 space-y-4" on:submit|preventDefault={register}>
            <h2 class="text-2xl font-bold text-center">{isSignup ? 'Register' : "Login"}</h2>

            <label class="form-control">
                <div class="label"><span class="label-text">Username</span></div>
                <input
                        type="text"
                        class="input input-bordered w-full"
                        bind:value={username}
                        autocomplete="username"
                        required
                />
            </label>

            <label class="form-control">
                <div class="label"><span class="label-text">Password</span></div>
                <input
                        type="password"
                        class="input input-bordered w-full"
                        bind:value={password}
                        autocomplete="new-password"
                        required
                />
            </label>

            <button class="btn btn-primary w-full" type="submit" disabled={loading}>
                {isSignup ? 'Register' : "Login"}
            </button>

            {#if loading}
                <span>Loading...</span>
            {/if}

            {#if response}
                <div class="alert mt-2">
                    <span class="break-all">{response}</span>
                </div>
            {/if}
        </form>

        <button on:click={toggleSignup} class="text-sm text-center link-primary cursor-pointer">
            {isSignup ? 'Already have an account? Log in' : "Don't have an account? Register"}
        </button>
    </div>
</div>
