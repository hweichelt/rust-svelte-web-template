import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

/** Signed-in users have no business on the login/register pages. */
export const load: LayoutServerLoad = ({ locals }) => {
	if (locals.user) redirect(303, '/');
	return {};
};
