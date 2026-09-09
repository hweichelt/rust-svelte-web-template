import { fail, redirect } from '@sveltejs/kit';
import { api } from '$lib/server/api';
import {
	sessionMaxAgeFromResponse,
	sessionTokenFromResponse,
	setSessionCookie
} from '$lib/server/session';
import type { User } from '$lib/types';
import type { Actions } from './$types';

export const actions: Actions = {
	default: async ({ request, fetch, cookies, url }) => {
		const form = await request.formData();
		const email = String(form.get('email') ?? '').trim();
		const password = String(form.get('password') ?? '');

		const result = await api<User>(fetch, '/api/auth/login', {
			method: 'POST',
			body: { email, password }
		});

		if (!result.ok) {
			const message = result.status === 401 ? 'Wrong email or password.' : result.error.message;
			return fail(result.status, { email, message });
		}

		const token = sessionTokenFromResponse(result.response);
		if (!token) return fail(502, { email, message: 'The server did not return a session.' });
		setSessionCookie(cookies, token, sessionMaxAgeFromResponse(result.response));

		redirect(303, safeRedirect(url.searchParams.get('redirectTo')));
	}
};

/** Only allow same-origin paths as a post-login destination. */
function safeRedirect(target: string | null): string {
	return target && target.startsWith('/') && !target.startsWith('//') ? target : '/';
}
