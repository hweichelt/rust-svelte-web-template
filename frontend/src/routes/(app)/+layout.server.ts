import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

/** Everything under (app) requires a signed-in user. */
export const load: LayoutServerLoad = ({ locals, url }) => {
	if (!locals.user) {
		const redirectTo =
			url.pathname === '/' ? '' : `?redirectTo=${encodeURIComponent(url.pathname)}`;
		redirect(303, `/login${redirectTo}`);
	}
	return { user: locals.user };
};
