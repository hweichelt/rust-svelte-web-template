<script lang="ts">
	import * as Card from '$lib/components/ui/card/index.js';

	let { data } = $props();
</script>

<h1 class="mb-6 text-2xl font-semibold tracking-tight">Hi, {data.user.display_name}</h1>

<Card.Root>
	<Card.Header>
		<Card.Title>Dashboard</Card.Title>
		<Card.Description>You are signed in. Build your app from here.</Card.Description>
	</Card.Header>
	<Card.Content>
		<p class="text-sm text-muted-foreground">Nothing here yet.</p>
	</Card.Content>
</Card.Root>
