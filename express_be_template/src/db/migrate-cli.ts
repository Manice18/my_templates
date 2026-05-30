import Logger from "../config/logger";
import { closePool, getPool } from "./pool";
import { runMigrations } from "./migrate";

async function main(): Promise<void> {
  await runMigrations(getPool());
  Logger.info("Migrations complete");
  await closePool();
}

main().catch((error) => {
  Logger.error("Migration failed", { error });
  process.exit(1);
});
