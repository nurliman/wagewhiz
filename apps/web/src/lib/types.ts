export type EmployeeStatus = "Permanent" | "Contract" | "Probation";

export type Nullable<T> = T | null | undefined;

export type PaginationParams = {
  page?: number;
  pageSize?: number;
};
