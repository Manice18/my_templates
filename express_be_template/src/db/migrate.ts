import fs from "node:fs";
import path from "node:path";

import type pg from "pg";

import Logger from "../config/logger";

const MIGRATIONS_DIR = path.join(process.cwd(), "db/migrations");

export async function runMigrations(pool: pg.Pool): Promise<void> {
  await pool.query(`
    CREATE TABLE IF NOT EXISTS _migrations (
      id SERIAL PRIMARY KEY,
      name TEXT NOT NULL UNIQUE,
      applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    )
  `);

  if (!fs.existsSync(MIGRATIONS_DIR)) {
    Logger.warn("Migrations directory not found", { path: MIGRATIONS_DIR });
    return;
  }

  const files = fs
    .readdirSync(MIGRATIONS_DIR)
    .filter((file) => file.endsWith(".sql"))
    .sort();

  for (const file of files) {
    const { rows } = await pool.query<{ exists: boolean }>(
      "SELECT EXISTS(SELECT 1 FROM _migrations WHERE name = $1) AS exists",
      [file],
    );

    if (rows[0]?.exists) {
      continue;
    }

    const sql = fs.readFileSync(path.join(MIGRATIONS_DIR, file), "utf-8");
    const client = await pool.connect();

    try {
      await client.query("BEGIN");
      await client.query(sql);
      await client.query("INSERT INTO _migrations (name) VALUES ($1)", [file]);
      await client.query("COMMIT");
      Logger.info("Applied migration", { file });
    } catch (error) {
      await client.query("ROLLBACK");
      throw error;
    } finally {
      client.release();
    }
  }
}
