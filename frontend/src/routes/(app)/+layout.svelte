<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { logout } from '$lib/auth';
	import { Button } from '$lib/components/ui/button/index.js';

	let { data, children } = $props();
	let loggingOut = $state(false);

	async function onLogout() {
		loggingOut = true;
		try {
			// Best effort: the backend clears the cookie; if it is unreachable the
			// next page load will find the session gone or still valid, either
			// way the user ends up where they should be.
			await logout().catch(() => undefined);
		} finally {
			loggingOut = false;
		}
		await goto(resolve('/login'), { invalidateAll: true });
	}
</script>

<header class="border-b">
	<div class="mx-auto flex h-14 max-w-5xl items-center justify-between px-4">
		<a href={resolve('/')} class="font-semibold tracking-tight">myapp</a>
		<div class="flex items-center gap-4">
			<span class="text-sm text-muted-foreground">{data.user.display_name}</span>
			<Button variant="outline" size="sm" onclick={onLogout} disabled={loggingOut}>Log out</Button>
		</div>
	</div>
</header>

<main class="mx-auto max-w-5xl px-4 py-8">
	{@render children()}
</main>
