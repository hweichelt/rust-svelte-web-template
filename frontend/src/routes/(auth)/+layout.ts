import { redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';

/** Signed-in users have no business on the login/register pages. */
export const load: LayoutLoad = async ({ parent }) => {
	const { user } = await parent();
	if (user) redirect(303, '/');
	return {};
};
