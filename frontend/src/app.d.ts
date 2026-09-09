import type { User } from '$lib/types';

// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			/** The signed-in user, resolved once per request in hooks.server.ts. */
			user: User | null;
			/** Raw session token from the cookie, forwarded to the backend as a bearer token. */
			token: string | null;
		}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
