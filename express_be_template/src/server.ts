import app from "./app";
import { config } from "./config/env";
import Logger from "./config/logger";
import { checkDatabaseConnection, closePool } from "./db";

const port = config.PORT;

async function start(): Promise<void> {
  await checkDatabaseConnection();
  Logger.info("Database connected");

  const server = app.listen(port, () => {
    Logger.info("Server started", {
      port,
      url: `http://localhost:${port}`,
      environment: config.ENVIRONMENT,
    });
  });

  const shutdown = (signal: NodeJS.Signals) => {
    Logger.warn("Shutdown signal received", { signal });
    server.close(async (error?: Error) => {
      if (error) {
        Logger.error("Error during server shutdown", { error });
        process.exit(1);
      }

      try {
        await closePool();
        Logger.info("Server closed gracefully");
        process.exit(0);
      } catch (closeError) {
        Logger.error("Error closing database pool", { error: closeError });
        process.exit(1);
      }
    });
  };

  process.on("SIGINT", shutdown);
  process.on("SIGTERM", shutdown);
}

start().catch((error) => {
  Logger.error("Failed to start server", { error });
  process.exit(1);
});
