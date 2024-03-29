export type EmployeeStatus = "Permanent" | "Contract" | "Probation";

export type Nullable<T> = T | null | undefined;

export type PaginationParams = {
  page?: number;
  page_size?: number;
};

export type PaginationResponse<T> = {
  data: T[];
  page: number;
  page_size: number;
  total: number;
};

export type WithInitials = {
  initials: string;
};

export type UserAccount = {
  id: string;
  person_id: string | null;
  role: string;
  username: string;
  created_at: string;
  updated_at: string;
  deleted_at: Date | null;
};

export interface Credential {
  access_token: string;
  refresh_token: string;
}

export type UserWithCredential = UserAccount & {
  credential: Credential;
};

export type Person = {
  id: string;
  created_at: Date;
  updated_at: Date;
  deleted_at: Date | null;
  name: string | null;
  nip: string | null;
  country: string | null;
  city: string | null;
  address: string | null;
  zip_code: string | null;
  email: string | null;
  phone: string | null;
  birthday: Date | null;
  organization: string | null;
  role: string | null;
  department: string | null;
  joining_date: Date | null;
  gender: string | null;
  status: EmployeeStatus | null;
  avatar_url: string | null;
};
