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
	default: async ({ request, fetch, cookies }) => {
		const form = await request.formData();
		const email = String(form.get('email') ?? '').trim();
		const display_name = String(form.get('display_name') ?? '').trim();
		const password = String(form.get('password') ?? '');
		const values = { email, display_name };

		if (password !== String(form.get('password_confirm') ?? '')) {
			return fail(400, { ...values, message: 'Passwords do not match.' });
		}

		const result = await api<User>(fetch, '/api/auth/register', {
			method: 'POST',
			body: { email, display_name, password }
		});

		if (!result.ok) {
			return fail(result.status, { ...values, message: result.error.message });
		}

		const token = sessionTokenFromResponse(result.response);
		if (!token) return fail(502, { ...values, message: 'The server did not return a session.' });
		setSessionCookie(cookies, token, sessionMaxAgeFromResponse(result.response));

		redirect(303, '/');
	}
};
