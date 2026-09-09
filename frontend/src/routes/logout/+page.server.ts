import { redirect } from '@sveltejs/kit';
import { api } from '$lib/server/api';
import { clearSessionCookie } from '$lib/server/session';
import type { Actions, PageServerLoad } from './$types';

/** GET /logout is not a page; send people home. */
export const load: PageServerLoad = () => {
	redirect(303, '/');
};

export const actions: Actions = {
	default: async ({ fetch, cookies, locals }) => {
		if (locals.token) {
			// Best effort: the cookie is cleared even if the backend is unreachable.
			await api(fetch, '/api/auth/logout', { method: 'POST', token: locals.token }).catch(
				() => undefined
			);
		}
		clearSessionCookie(cookies);
		redirect(303, '/login');
	}
};
