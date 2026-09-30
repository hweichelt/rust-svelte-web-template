import { api } from '$lib/api';
import type { LoginRequest, RegisterRequest, UserResponse } from '$lib/bindings/api';

export function me(fetchFn?: typeof fetch) {
	return api<UserResponse>('/api/auth/me', { fetch: fetchFn });
}

export function login(body: LoginRequest) {
	return api<UserResponse>('/api/auth/login', { method: 'POST', body });
}

export function register(body: RegisterRequest) {
	return api<UserResponse>('/api/auth/register', { method: 'POST', body });
}

export function logout() {
	return api<void>('/api/auth/logout', { method: 'POST' });
}

/** Only allow same-origin paths as a post-login destination. */
export function safeRedirect(target: string | null): string {
	return target && target.startsWith('/') && !target.startsWith('//') ? target : '/';
}
