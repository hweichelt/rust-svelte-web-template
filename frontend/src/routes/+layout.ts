import { me } from '$lib/auth';
import type { LayoutLoad } from './$types';

// The backend serves this app as a static bundle, so every page renders in
// the browser. Nothing here runs on a server.
export const ssr = false;

/** Resolve the signed-in user once per navigation; child layouts gate on it. */
export const load: LayoutLoad = async ({ fetch }) => {
	const result = await me(fetch);
	return { user: result.ok ? result.data : null };
};
