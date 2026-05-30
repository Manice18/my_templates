import { Router } from "express";

import { config } from "../config/env";

export function makeHealthRouter(): Router {
  const router = Router();

  router.get("/api/health", (_req, res) => {
    res.json({
      ok: true,
      service: config.SERVICE_NAME,
      environment: config.ENVIRONMENT,
    });
  });

  return router;
}
