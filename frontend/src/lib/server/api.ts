import { env } from '$env/dynamic/private';
import type { ApiErrorBody } from '$lib/types';

const API_URL = (env.API_URL ?? 'http://127.0.0.1:3000').replace(/\/$/, '');

export type ApiResult<T> =
	| { ok: true; status: number; data: T; response: Response }
	| { ok: false; status: number; error: ApiErrorBody };

type ApiOptions = {
	method?: 'GET' | 'POST' | 'PUT' | 'PATCH' | 'DELETE';
	body?: unknown;
	token?: string | null;
};

/**
 * Call the backend from the SvelteKit server. Pass the event's `fetch` so
 * SvelteKit can trace the request. Backend errors come back as `ok: false`
 * with the backend's `{code, message}`; network failures throw.
 */
export async function api<T = unknown>(
	fetchFn: typeof fetch,
	path: string,
	{ method = 'GET', body, token }: ApiOptions = {}
): Promise<ApiResult<T>> {
	const headers: Record<string, string> = { accept: 'application/json' };
	if (body !== undefined) headers['content-type'] = 'application/json';
	if (token) headers.authorization = `Bearer ${token}`;

	const response = await fetchFn(`${API_URL}${path}`, {
		method,
		headers,
		body: body === undefined ? undefined : JSON.stringify(body)
	});

	if (response.ok) {
		const data = (response.status === 204 ? undefined : await response.json()) as T;
		return { ok: true, status: response.status, data, response };
	}

	let error: ApiErrorBody = { code: 'unknown', message: response.statusText || 'Request failed' };
	try {
		const parsed = (await response.json()) as { error?: ApiErrorBody };
		if (parsed.error) error = parsed.error;
	} catch {
		// Non-JSON error body; keep the fallback.
	}
	return { ok: false, status: response.status, error };
}

export { API_URL };
