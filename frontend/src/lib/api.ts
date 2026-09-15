import type { ErrorBody, ErrorCode } from '$lib/bindings/api';

/** The backend's error detail, or `unknown` when the response had no JSON error body. */
export type ApiError = { code: ErrorCode | 'unknown'; message: string };

export type ApiResult<T> =
	{ ok: true; status: number; data: T } | { ok: false; status: number; error: ApiError };

type ApiOptions = {
	method?: 'GET' | 'POST' | 'PUT' | 'PATCH' | 'DELETE';
	body?: unknown;
	/** Pass the `fetch` given to a `load` function so SvelteKit can track it. */
	fetch?: typeof fetch;
};

/**
 * Call the backend. The app is served from the same origin as the API, so
 * relative paths work and the session cookie travels automatically.
 * Backend errors come back as `ok: false` with the backend's `{code, message}`;
 * network failures throw.
 */
export async function api<T = unknown>(
	path: string,
	{ method = 'GET', body, fetch: fetchFn = fetch }: ApiOptions = {}
): Promise<ApiResult<T>> {
	const headers: Record<string, string> = { accept: 'application/json' };
	if (body !== undefined) headers['content-type'] = 'application/json';

	const response = await fetchFn(path, {
		method,
		headers,
		body: body === undefined ? undefined : JSON.stringify(body)
	});

	if (response.ok) {
		const data = (response.status === 204 ? undefined : await response.json()) as T;
		return { ok: true, status: response.status, data };
	}

	let error: ApiError = { code: 'unknown', message: response.statusText || 'Request failed' };
	try {
		const parsed = (await response.json()) as Partial<ErrorBody>;
		if (parsed.error) error = parsed.error;
	} catch {
		// Non-JSON error body; keep the fallback.
	}
	return { ok: false, status: response.status, error };
}
