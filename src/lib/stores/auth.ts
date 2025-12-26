import { writable } from 'svelte/store';

export interface User {
    id: string;
    name: string;
    email: string;
}

export const user = writable<User | null>(null);
export const isLoggedIn = writable(false);
