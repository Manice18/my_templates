import dotenv from "dotenv";
import { z } from "zod";

dotenv.config();

const envSchema = z.object({
  ENVIRONMENT: z.enum(["development", "production"]).default("development"),

  PORT: z.coerce.number().default(8080),

  SERVICE_NAME: z.string().default("express_be"),

  FE_ORIGIN: z.url().default("http://localhost:3000"),

  LOG_LEVEL: z.enum(["error", "warn", "info", "http", "debug"]).default("info"),

  LOG_DIRECTORY: z.string().default("logs"),
});

const parsed = envSchema.safeParse(process.env);

if (!parsed.success) {
  const formatted = z.treeifyError(parsed.error);

  console.error(
    "Invalid environment variables:",
    JSON.stringify(formatted, null, 2),
  );

  process.exit(1);
}

export const config = parsed.data;

export type AppConfig = typeof config;
