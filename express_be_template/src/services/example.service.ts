import {
  getExampleById,
  listExamples,
} from "../db/queries/example.queries";

export type ExampleItem = {
  id: string;
  name: string;
  createdAt: string;
};

export async function listItems(): Promise<ExampleItem[]> {
  const rows = await listExamples();

  return rows.map((row) => ({
    id: row.id,
    name: row.name,
    createdAt: row.created_at.toISOString(),
  }));
}

export async function getItemById(id: string): Promise<ExampleItem | undefined> {
  const row = await getExampleById(id);

  if (!row) {
    return undefined;
  }

  return {
    id: row.id,
    name: row.name,
    createdAt: row.created_at.toISOString(),
  };
}
