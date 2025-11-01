import { browser } from '$app/environment';
import { writable } from 'svelte/store';

function createAuth() {
    const initial = browser ? localStorage.getItem('jwt') : null;
    const { subscribe, set } = writable<string | null>(initial);

    if (browser) {
        subscribe((value) => {
            if (value) localStorage.setItem('jwt', value);
            else localStorage.removeItem('jwt');
        });
    }

    return {
        subscribe,
        setToken: (t: string) => set(t),
        clear: () => set(null)
    };
}

export const auth = createAuth();
