import type { Cookies } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

/** Must match the cookie name the backend issues. */
export const SESSION_COOKIE = 'myapp_session';

/**
 * Pull the session token out of the backend's `Set-Cookie` headers. The
 * SvelteKit server then re-issues it on its own origin so the browser only
 * ever talks to the frontend.
 */
export function sessionTokenFromResponse(response: Response): string | null {
	for (const header of response.headers.getSetCookie()) {
		const [pair] = header.split(';');
		const [name, ...rest] = pair.split('=');
		if (name.trim() === SESSION_COOKIE) return rest.join('=').trim() || null;
	}
	return null;
}

/**
 * Whether to mark the cookie `Secure`. SvelteKit's default only allows
 * non-secure cookies on `localhost`; explicit config lets plain-http access
 * from other addresses (e.g. 127.0.0.1 or a LAN host) work, and lets a
 * deployment behind HTTPS opt in.
 */
function cookieSecure(): boolean {
	return ['1', 'true', 'yes'].includes((env.COOKIE_SECURE ?? 'false').toLowerCase());
}

export function setSessionCookie(cookies: Cookies, token: string, maxAgeSeconds: number) {
	cookies.set(SESSION_COOKIE, token, {
		path: '/',
		httpOnly: true,
		sameSite: 'lax',
		secure: cookieSecure(),
		maxAge: maxAgeSeconds
	});
}

export function clearSessionCookie(cookies: Cookies) {
	cookies.delete(SESSION_COOKIE, { path: '/', secure: cookieSecure() });
}

/** Read `Max-Age` from the backend cookie so the frontend cookie expires at the same time. */
export function sessionMaxAgeFromResponse(
	response: Response,
	fallback = 60 * 60 * 24 * 30
): number {
	for (const header of response.headers.getSetCookie()) {
		const match = /max-age=(\d+)/i.exec(header);
		if (match) return Number(match[1]);
	}
	return fallback;
}
