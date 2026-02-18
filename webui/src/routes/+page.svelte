<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Accordion from '$lib/components/ui/accordion/index.js';
	import * as Field from '$lib/components/ui/field/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import * as Select from '$lib/components/ui/select/index.js';
	import { Textarea } from '$lib/components/ui/textarea/index.js';

	import { type SuperValidated, type Infer, superForm, fileProxy } from 'sveltekit-superforms';
	import { zod4Client } from 'sveltekit-superforms/adapters';
	import * as Form from '$lib/components/ui/form/index.js';
	import { jobCreateSchema, type JobCreate, type Model } from '$lib/job';

	// Mock model list – replace with real API call if needed
	let models: { open_source: Model[] } = {
		open_source: [
			{
				id: 'openai/gpt-oss-20b',
				name: 'OpenAI gpt-oss',
				params: '20b'
			},
			{
				id: 'Qwen/Qwen3-30B-A3B-Thinking-2507-FP8',
				name: 'Qwen3',
				params: '30b a3b'
			}
		]
	};

	let { data }: { data: { form: SuperValidated<Infer<JobCreate>> } } = $props();
	const form = superForm(data.form, {
		validators: zod4Client(jobCreateSchema)
	});
	const { form: formData, enhance } = form;
	const modelSelection = $derived(
		models.open_source.find((m) => m.id === $formData.model)?.name ?? 'Select a model'
	);
	const input_file = fileProxy(formData, 'input_file');
</script>

<form method="post" enctype="multipart/form-data" use:enhance class="mx-auto w-full max-w-xl">
	<Field.Set>
		<Field.Legend>Create a Job</Field.Legend>
		<Field.Description>Submit a translation job via this form</Field.Description>
		<Field.Separator />
		<Field.Group class="grid grid-cols-1 @md:grid-cols-2">
			<Form.Field {form} name="input_file" class="@md:col-span-2">
				<Form.Control>
					{#snippet children({ props })}
						<Form.Label>Input document</Form.Label>
						<Input {...props} bind:files={$input_file} type="file" />
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>

			<Form.Field {form} name="source_lang">
				<Form.Control>
					{#snippet children({ props })}
						<Form.Label>Source language</Form.Label>
						<Input {...props} bind:value={$formData.source_lang} placeholder="e.g., English" />
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>

			<Form.Field {form} name="target_lang">
				<Form.Control>
					{#snippet children({ props })}
						<Form.Label>Target language</Form.Label>
						<Input {...props} bind:value={$formData.target_lang} placeholder="e.g., Chinese" />
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>

			<Form.Field {form} class="@md:col-span-2" name="model">
				<Form.Control>
					{#snippet children({ props })}
						<Form.Label>Model</Form.Label>
						<Select.Root type="single" name="model" bind:value={$formData.model}>
							<Select.Trigger class="w-full">{modelSelection}</Select.Trigger>
							<Select.Content>
								<Select.Group>
									<Select.Label>Open source models</Select.Label>
									{#each models.open_source as m}
										<Select.Item value={m.id}>{m.name} <span class="">{m.params}</span></Select.Item
										>
									{/each}
								</Select.Group>
								<Select.Group>
									<Select.Label>Proprietary models</Select.Label>
								</Select.Group>
							</Select.Content>
						</Select.Root>
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>
		</Field.Group>

		<Field.Separator />
		<Field.Group>
			<Accordion.Root>
				<Accordion.Item>
					<Form.Field {form} name="system_prompt">
						<Form.Control>
							{#snippet children({ props })}
								<Accordion.Trigger><Form.Label>System Prompt</Form.Label></Accordion.Trigger>
								<Accordion.Content>
									<Form.Description>
										The prompt that will come before every text. Equivalent to <code>system</code> role
										in chat completions.
									</Form.Description>
									<Textarea
										{...props}
										bind:value={$formData.system_prompt}
										rows={5}
										name="system_prompt"
									/>
								</Accordion.Content>
							{/snippet}
						</Form.Control>
					</Form.Field>
				</Accordion.Item>
				<Accordion.Item>
					<Form.Field {form} name="user_prompt">
						<Form.Control>
							{#snippet children({ props })}
								<Accordion.Trigger><Form.Label>User Prompt</Form.Label></Accordion.Trigger>
								<Accordion.Content>
									<Form.Description>
										The prompt that is a part of each text. Equivalent to <code>user</code> role in chat
										completions.
									</Form.Description>
									<Textarea
										{...props}
										bind:value={$formData.user_prompt}
										rows={5}
										name="user_prompt"
									/>
								</Accordion.Content>
							{/snippet}
						</Form.Control>
					</Form.Field>
				</Accordion.Item>
			</Accordion.Root>
		</Field.Group>
		<Field.Separator />
		<Form.Button>Submit job</Form.Button>
	</Field.Set>
</form>
