<script lang="ts">
	import { enhance } from '$app/forms';
	import { resolve } from '$app/paths';
	import { Button } from '$lib/components/ui/button/index.js';

	let { data, children } = $props();
</script>

<header class="border-b">
	<div class="mx-auto flex h-14 max-w-5xl items-center justify-between px-4">
		<a href={resolve('/')} class="font-semibold tracking-tight">myapp</a>
		<div class="flex items-center gap-4">
			<span class="text-sm text-muted-foreground">{data.user.display_name}</span>
			<form method="POST" action="/logout" use:enhance>
				<Button type="submit" variant="outline" size="sm">Log out</Button>
			</form>
		</div>
	</div>
</header>

<main class="mx-auto max-w-5xl px-4 py-8">
	{@render children()}
</main>
