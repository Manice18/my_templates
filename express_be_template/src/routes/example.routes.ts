import { Router } from "express";

import { ApiError } from "../errors/errors";
import { getItemById, listItems } from "../services/example.service";

export function makeExampleRouter(): Router {
  const router = Router();

  router.get("/api/examples", (_req, res) => {
    res.json({ items: listItems() });
  });

  router.get("/api/examples/:id", (req, res, next) => {
    try {
      const item = getItemById(req.params.id);
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
