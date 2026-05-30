import pg from "pg";

import { config } from "../config/env";

const { Pool } = pg;

let pool: pg.Pool | undefined;

export function getPool(): pg.Pool {
  if (!pool) {
    pool = new Pool({
      connectionString: config.DATABASE_URL,
      max: config.DB_MAX_CONNECTIONS,
      connectionTimeoutMillis: config.DB_CONNECTION_TIMEOUT * 1000,
    });
  }

  return pool;
}

export async function checkDatabaseConnection(): Promise<void> {
  await getPool().query("SELECT 1");
}

export async function closePool(): Promise<void> {
  if (pool) {
    await pool.end();
    pool = undefined;
  }
}
