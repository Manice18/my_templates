import express from "express";
import cors from "cors";

import { ApiError } from "./errors/errors";
import Logger from "./config/logger";
import { config } from "./config/env";
import { requestLogger } from "./middleware/requestLogger";
import { registerRoutes } from "./routes";

process.on("uncaughtException", (e) => {
  Logger.error(e);
});

const app = express();

app.use(requestLogger);
app.use(express.json());
app.use(express.urlencoded({ extended: true }));
app.use(cors({ origin: config.FE_ORIGIN }));

registerRoutes(app);

app.use(
  (
    err: unknown,
    req: express.Request,
    res: express.Response,
    _next: express.NextFunction,
  ) => {
    if (err instanceof ApiError) {
      Logger.warn("Handled API error", {
        name: err.name,
        message: err.message,
        statusCode: err.status,
        path: req.originalUrl,
        method: req.method,
      });
      res.status(err.status).json({ error: err.message });
      return;
    }

    Logger.error("Unhandled application error", {
      error:
        err instanceof Error
          ? { name: err.name, message: err.message, stack: err.stack }
          : err,
      path: req.originalUrl,
      method: req.method,
    });

    const message =
      err instanceof Error ? err.message : "Internal server error";
    res.status(500).json({ error: message });
  },
);

export default app;
