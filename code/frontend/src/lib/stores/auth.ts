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

function createUsername() {
    const initial = browser ? localStorage.getItem('username') : null;
    const { subscribe, set } = writable<string | null>(initial);

    if (browser) {
        subscribe((value) => {
            if (value) localStorage.setItem('username', value);
            else localStorage.removeItem('username');
        });
    }

    return {
        subscribe,
        setName: (n: string) => set(n),
        clear: () => set(null)
    };
}

export const auth = createAuth();
export const username = createUsername();
