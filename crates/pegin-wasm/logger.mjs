/**
 * Winston logger for the pegin Node scripts (see wiki: logging-strategy).
 * Errors always; info/debug only when LOG_LEVEL is raised. All levels go to
 * stderr so stdout stays reserved for program output (results, reports).
 */
import winston from "winston";

export const logger = winston.createLogger({
  level: process.env.LOG_LEVEL ?? "error",
  format: winston.format.combine(
    winston.format.colorize(),
    winston.format.printf(({ level, message }) => `${level} ${message}`)
  ),
  transports: [
    new winston.transports.Console({
      stderrLevels: ["error", "warn", "info", "debug"],
    }),
  ],
});
