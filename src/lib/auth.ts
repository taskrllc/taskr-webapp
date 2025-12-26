import { user } from './stores/auth';

export async function login(email: string, password: string) {
    const res = await fetch('/api/login', {
        method: 'POST',
        body: JSON.stringify({ email, password }),
        headers: { 'Content-Type': 'application/json' }
    });
    if (res.ok) {
        const data = await res.json();
        user.set(data.user);
        return true;
    }
    return false;
}

export function logout() {
    user.set(null);
    fetch('/api/logout');
}
