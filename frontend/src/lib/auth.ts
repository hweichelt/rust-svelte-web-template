import { api } from '$lib/api';
import type { User } from '$lib/types';

export function me(fetchFn?: typeof fetch) {
	return api<User>('/api/auth/me', { fetch: fetchFn });
}

export function login(body: { email: string; password: string }) {
	return api<User>('/api/auth/login', { method: 'POST', body });
}

export function register(body: { email: string; display_name: string; password: string }) {
	return api<User>('/api/auth/register', { method: 'POST', body });
}

export function logout() {
	return api<void>('/api/auth/logout', { method: 'POST' });
}

/** Only allow same-origin paths as a post-login destination. */
export function safeRedirect(target: string | null): string {
	return target && target.startsWith('/') && !target.startsWith('//') ? target : '/';
}
