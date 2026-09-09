import type { RequestHandler } from '@sveltejs/kit';
import { API_URL } from '$lib/server/api';

/**
 * Same-origin proxy for browser-side API calls. Forwards `/api/<path>` to the
 * backend and attaches the session token as a bearer header, so client code
 * never needs to know about cookies, CORS, or the backend's address.
 */
const proxy: RequestHandler = async ({ request, params, url, locals, fetch }) => {
	const headers = new Headers();
	const contentType = request.headers.get('content-type');
	if (contentType) headers.set('content-type', contentType);
	headers.set('accept', request.headers.get('accept') ?? 'application/json');
	if (locals.token) headers.set('authorization', `Bearer ${locals.token}`);

	const hasBody = request.method !== 'GET' && request.method !== 'HEAD';
	const upstream = await fetch(`${API_URL}/api/${params.path}${url.search}`, {
		method: request.method,
		headers,
		body: hasBody ? await request.arrayBuffer() : undefined
	});

	const responseHeaders = new Headers();
	const upstreamType = upstream.headers.get('content-type');
	if (upstreamType) responseHeaders.set('content-type', upstreamType);
	// Never forward Set-Cookie: sessions are issued via the login/register actions.
	return new Response(upstream.body, { status: upstream.status, headers: responseHeaders });
};

export const GET = proxy;
export const POST = proxy;
export const PUT = proxy;
export const PATCH = proxy;
export const DELETE = proxy;
