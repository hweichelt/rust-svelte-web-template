import { redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';

/** Everything under (app) requires a signed-in user. */
export const load: LayoutLoad = async ({ parent, url }) => {
	const { user } = await parent();
	if (!user) {
		const redirectTo =
			url.pathname === '/' ? '' : `?redirectTo=${encodeURIComponent(url.pathname + url.search)}`;
		redirect(303, `/login${redirectTo}`);
	}
	return { user };
};
