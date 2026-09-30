<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { login, safeRedirect } from '$lib/auth';
	import * as Alert from '$lib/components/ui/alert/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';

	let email = $state('');
	let password = $state('');
	let message = $state<string | null>(null);
	let submitting = $state(false);

	async function onsubmit(event: SubmitEvent) {
		event.preventDefault();
		submitting = true;
		message = null;
		try {
			const result = await login({ email: email.trim(), password });
			if (!result.ok) {
				message = result.status === 401 ? 'Wrong email or password.' : result.error.message;
				return;
			}
			// Re-run the layout loads so the signed-in user is picked up.
			// eslint-disable-next-line svelte/no-navigation-without-resolve -- validated same-origin path
			await goto(safeRedirect(page.url.searchParams.get('redirectTo')), { invalidateAll: true });
		} catch {
			message = 'Could not reach the server.';
		} finally {
			submitting = false;
		}
	}
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>Log in</Card.Title>
		<Card.Description>Welcome back.</Card.Description>
	</Card.Header>
	<Card.Content>
		<form class="grid gap-4" {onsubmit}>
			{#if message}
				<Alert.Root variant="destructive">
					<Alert.Description>{message}</Alert.Description>
				</Alert.Root>
			{/if}
			<div class="grid gap-2">
				<Label for="email">Email</Label>
				<Input
					id="email"
					name="email"
					type="email"
					autocomplete="email"
					required
					bind:value={email}
				/>
			</div>
			<div class="grid gap-2">
				<Label for="password">Password</Label>
				<Input
					id="password"
					name="password"
					type="password"
					autocomplete="current-password"
					required
					bind:value={password}
				/>
			</div>
			<Button type="submit" class="w-full" disabled={submitting}>
				{submitting ? 'Logging in…' : 'Log in'}
			</Button>
		</form>
	</Card.Content>
	<Card.Footer class="justify-center text-sm">
		<span class="text-muted-foreground">No account yet?</span>
		<a href="{resolve('/register')}{page.url.search}" class="ml-1 underline underline-offset-4"
			>Register</a
		>
	</Card.Footer>
</Card.Root>
