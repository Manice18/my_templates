import type { Express } from "express";

import { makeHealthRouter } from "./health.routes";
import { makeExampleRouter } from "./example.routes";

export function registerRoutes(app: Express): void {
  app.use(makeHealthRouter());
  app.use(makeExampleRouter());
}
