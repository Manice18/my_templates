import { Router } from "express";

import { config } from "../config/env";
import { checkDatabaseConnection } from "../db";

export function makeHealthRouter(): Router {
  const router = Router();

  router.get("/api/health", async (_req, res) => {
    let database: "ok" | "error" = "ok";

    try {
      await checkDatabaseConnection();
    } catch {
      database = "error";
    }

    const ok = database === "ok";

    res.status(ok ? 200 : 503).json({
      ok,
      service: config.SERVICE_NAME,
      environment: config.ENVIRONMENT,
      database,
    });
  });

  return router;
}
