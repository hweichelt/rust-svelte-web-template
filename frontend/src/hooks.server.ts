import type { Handle } from '@sveltejs/kit';
import { api } from '$lib/server/api';
import { SESSION_COOKIE, clearSessionCookie } from '$lib/server/session';
import type { User } from '$lib/types';

export const handle: Handle = async ({ event, resolve }) => {
	const token = event.cookies.get(SESSION_COOKIE) ?? null;
	event.locals.token = token;
	event.locals.user = null;

	if (token) {
		const result = await api<User>(event.fetch, '/api/auth/me', { token });
		if (result.ok) {
			event.locals.user = result.data;
		} else if (result.status === 401) {
			// Session expired or was revoked; drop the stale cookie.
			clearSessionCookie(event.cookies);
			event.locals.token = null;
		}
	}

	return resolve(event);
};
