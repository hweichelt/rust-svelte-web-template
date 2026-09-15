<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { register, safeRedirect } from '$lib/auth';
	import * as Alert from '$lib/components/ui/alert/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';

	let email = $state('');
	let displayName = $state('');
	let password = $state('');
	let passwordConfirm = $state('');
	let message = $state<string | null>(null);
	let submitting = $state(false);

	async function onsubmit(event: SubmitEvent) {
		event.preventDefault();
		message = null;
		if (password !== passwordConfirm) {
			message = 'Passwords do not match.';
			return;
		}
		submitting = true;
		try {
			const result = await register({
				email: email.trim(),
				display_name: displayName.trim(),
				password
			});
			if (!result.ok) {
				message = result.error.message;
				return;
			}
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
		<Card.Title>Create an account</Card.Title>
		<Card.Description>It only takes a moment.</Card.Description>
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
				<Label for="display_name">Display name</Label>
				<Input
					id="display_name"
					name="display_name"
					autocomplete="nickname"
					required
					maxlength={80}
					bind:value={displayName}
				/>
			</div>
			<div class="grid gap-2">
				<Label for="password">Password</Label>
				<Input
					id="password"
					name="password"
					type="password"
					autocomplete="new-password"
					required
					minlength={8}
					bind:value={password}
				/>
			</div>
			<div class="grid gap-2">
				<Label for="password_confirm">Confirm password</Label>
				<Input
					id="password_confirm"
					name="password_confirm"
					type="password"
					autocomplete="new-password"
					required
					bind:value={passwordConfirm}
				/>
			</div>
			<Button type="submit" class="w-full" disabled={submitting}>
				{submitting ? 'Creating account…' : 'Create account'}
			</Button>
		</form>
	</Card.Content>
	<Card.Footer class="justify-center text-sm">
		<span class="text-muted-foreground">Already have an account?</span>
		<a href="{resolve('/login')}{page.url.search}" class="ml-1 underline underline-offset-4"
			>Log in</a
		>
	</Card.Footer>
</Card.Root>
