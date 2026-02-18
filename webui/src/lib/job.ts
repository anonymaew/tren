import { z } from 'zod';
import type { ColumnDef } from "@tanstack/table-core";
import { createRawSnippet } from "svelte";
import { renderSnippet } from '$lib/components/ui/data-table/index.js';

export const default_system_prompt = `You are an expert translator. Please translate {{ source_language }} into {{ target_language }}. The user will submit sentences or paragraphs with some contexts; please only translate the intended text into {{ target_language }}.

- If there are symbols {{ special_tokens | join(\\" , \\") }}, keep the symbol intact on the result text in the correct position.
- Do not give any alternative translation or including any previous context, notes or discussion.`

export const default_user_prompt = `{%- set previous_chunks = previous_chunks[-8:] -%}
{%- if previous_chunks -%}
Given the previous context:

{{ previous_chunks | join(\\"\\n\\n\\") }}

Only translate the following text:

{% endif -%}
{{ source_text }}`;

const modelId = z.string().nonempty()

export const modelSchema = z.object({
	id: modelId,
	name: z.string().max(32),
	params: z.string(),
})

export type Model = z.infer<typeof modelSchema>;

// md, txt, docx; no pdf yet
const acceptedTypes: z.core.util.MimeTypes[] = [
	"text/markdown",
	"text/plain",
	"application/vnd.openxmlformats-officedocument.wordprocessingml.document"
];

export const jobCreateSchema = z.object({
	name: z.string().optional(),
	source_lang: z.string().nonempty(),
	target_lang: z.string().nonempty(),
	model: modelId,
	system_prompt: z.string().default(default_system_prompt),
	user_prompt: z.string().default(default_user_prompt),
	input_file: z.file().mime(acceptedTypes)
})

const jobSchema = z.object({
	...jobCreateSchema.shape,
	id: z.uuidv7(),
	created_at: z.date(),
	status: z.enum(["waiting", "processing", "successed", "failed"]),
	output_file: z.file().mime(acceptedTypes).optional()
})
	.transform(job => ({
		...job,
		name: job.name ?? `${parseInt(Date.now() / 1000).toString(16)}`
	}))

export type Job = z.output<typeof jobSchema>

export type JobCreate = z.input<typeof jobCreateSchema>

export const jobColumns: ColumnDef<Job>[] = [
	{
		accessorKey: "created_at",
		header: "Created at",
		cell: ({ row }) => {
			const amountCellSnippet = createRawSnippet<[{ created_at: Date }]>(
				(getDate) => {
					const { created_at: date } = getDate();
					return {
						render: () =>
							`<time datetime="${date.toISOString()}">${date.toLocaleString()}</div>`,
					};
				}
			);
			return renderSnippet(amountCellSnippet, {
				created_at: row.original.created_at,
			});
		},
	},
	{
		accessorKey: "name",
		header: "Name",
	},
	{
		accessorKey: "status",
		header: "Status",
	},
	{
		accessorKey: "source_lang",
		header: "Source language",
	},
	{
		accessorKey: "target_lang",
		header: "Target language",
	},
	{
		accessorKey: "model",
		header: "Model"
	}
];
