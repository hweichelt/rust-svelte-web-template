<script lang="ts">
	import { enhance } from '$app/forms';
	import { resolve } from '$app/paths';
	import * as Alert from '$lib/components/ui/alert/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';

	let { form } = $props();
	let submitting = $state(false);
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>Create an account</Card.Title>
		<Card.Description>Sign up to get started.</Card.Description>
	</Card.Header>
	<Card.Content>
		<form
			method="POST"
			class="grid gap-4"
			use:enhance={() => {
				submitting = true;
				return async ({ update }) => {
					await update();
					submitting = false;
				};
			}}
		>
			{#if form?.message}
				<Alert.Root variant="destructive">
					<Alert.Description>{form.message}</Alert.Description>
				</Alert.Root>
			{/if}
			<div class="grid gap-2">
				<Label for="display_name">Name</Label>
				<Input
					id="display_name"
					name="display_name"
					autocomplete="name"
					required
					maxlength={80}
					value={form?.display_name ?? ''}
				/>
			</div>
			<div class="grid gap-2">
				<Label for="email">Email</Label>
				<Input
					id="email"
					name="email"
					type="email"
					autocomplete="email"
					required
					value={form?.email ?? ''}
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
					minlength={8}
				/>
			</div>
			<Button type="submit" class="w-full" disabled={submitting}>
				{submitting ? 'Creating account…' : 'Create account'}
			</Button>
		</form>
	</Card.Content>
	<Card.Footer class="justify-center text-sm">
		<span class="text-muted-foreground">Already have an account?</span>
		<a href={resolve('/login')} class="ml-1 underline underline-offset-4">Log in</a>
	</Card.Footer>
</Card.Root>
