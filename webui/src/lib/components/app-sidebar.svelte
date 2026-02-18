<script lang="ts">
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';

	const items = [
		{ href: '/', label: 'Create a Job' },
		{ href: '/job', label: 'Monitor Jobs' }
	];
</script>

<Sidebar.Root>
	<Sidebar.Header>Translation Engine</Sidebar.Header>
	<Sidebar.Content>
		<Sidebar.Group>
			<Sidebar.GroupLabel>Jobs</Sidebar.GroupLabel>
			<Sidebar.GroupContent>
				<Sidebar.Menu>
					{#each items as item}
						<Sidebar.MenuItem>
							<Sidebar.MenuButton>
								{#snippet child({ props })}
									<a href={item.href} {...props}>{item.label}</a>
								{/snippet}
							</Sidebar.MenuButton>
						</Sidebar.MenuItem>
					{/each}
				</Sidebar.Menu>
			</Sidebar.GroupContent>
		</Sidebar.Group>
	</Sidebar.Content>
	<Sidebar.Footer></Sidebar.Footer>
</Sidebar.Root>
