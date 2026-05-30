import { Router } from "express";

import { ApiError } from "../errors/errors";
import { getItemById, listItems } from "../services/example.service";

export function makeExampleRouter(): Router {
  const router = Router();

  router.get("/api/examples", async (_req, res, next) => {
    try {
      const items = await listItems();
      res.json({ items });
    } catch (error) {
      next(error);
    }
  });

  router.get("/api/examples/:id", async (req, res, next) => {
    try {
      const item = await getItemById(req.params.id);
      if (!item) {
        throw new ApiError("Item not found", 404);
      }
      res.json({ item });
    } catch (error) {
      next(error);
    }
  });

  return router;
}
