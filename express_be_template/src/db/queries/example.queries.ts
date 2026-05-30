import { getPool } from "../pool";

export type ExampleRow = {
  id: string;
  name: string;
  created_at: Date;
};

export async function listExamples(): Promise<ExampleRow[]> {
  const { rows } = await getPool().query<ExampleRow>(
    "SELECT id, name, created_at FROM examples ORDER BY created_at",
  );

  return rows;
}

export async function getExampleById(id: string): Promise<ExampleRow | undefined> {
  const { rows } = await getPool().query<ExampleRow>(
    "SELECT id, name, created_at FROM examples WHERE id = $1",
    [id],
  );

  return rows[0];
}
