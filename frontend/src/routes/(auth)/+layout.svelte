<script lang="ts">
	let { children } = $props();
</script>

<main class="flex min-h-screen items-center justify-center px-4">
	<div class="w-full max-w-sm">
		<h1 class="mb-6 text-center text-2xl font-semibold tracking-tight">myapp</h1>
		{@render children()}
	</div>
</main>
