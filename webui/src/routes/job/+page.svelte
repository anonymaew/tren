<script lang="ts">
	import { type Job, jobColumns } from '$lib/job';
	import DataTable from '$lib/components/data-table.svelte';

	let jobs: Job[] = [
		{
			name: 'job1',
			source_lang: 'English',
			target_lang: 'Chinese',
			model: 'gpt-oss',
			system_prompt: 'something',
			user_prompt: 'something',
			input_file: new File([], 'test.txt'),
			id: '555',
			created_at: new Date('2026-02-17T10:47:06.384Z'),
			status: 'waiting'
		},
		{
			name: 'job2',
			source_lang: 'English',
			target_lang: 'Chinese',
			model: 'qwen3',
			system_prompt: 'something',
			user_prompt: 'something',
			input_file: new File([], 'test.txt'),
			id: '555',
			created_at: new Date('2026-02-17T09:57:06.384Z'),
			status: 'waiting'
		},
		{
			name: 'job3',
			source_lang: 'English',
			target_lang: 'Chinese',
			model: 'gpt-oss',
			system_prompt: 'something',
			user_prompt: 'something',
			input_file: new File([], 'test.txt'),
			id: '555',
			created_at: new Date('2026-02-17T09:47:06.384Z'),
			status: 'waiting'
		}
	];
</script>

<h1 class="mb-4 text-2xl font-bold">Job History</h1>

{#if jobs.length === 0}
	<p>No jobs submitted yet.</p>
{:else}
	<DataTable data={jobs} columns={jobColumns} />
{/if}
