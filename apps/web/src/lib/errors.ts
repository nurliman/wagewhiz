import { BaseError } from "make-error";

export type AppErrorType =
  | "UNAUTHENTICATED"
  | "FORBIDDEN"
  | "BAD_USER_INPUT"
  | "RESOURCE_NOT_FOUND"
  | "VALIDATION_ERROR"
  | "INTERNAL_SERVER_ERROR"
  | "UNKNOWN_ERROR";

export class AppError extends BaseError {
  constructor(
    message = "An error occurred",
    public code: AppErrorType = "UNKNOWN_ERROR",
  ) {
    super(message);
  }
}

export const isAppError = (error: unknown): error is AppError => {
  return error instanceof AppError;
};
