export type EmployeeStatus = "Permanent" | "Contract" | "Probation";

export type Nullable<T> = T | null | undefined;

export type Employee = {
  id: number;
  NIP: string;
  name: string;
  email: string;
  phone: string;
  department: string;
  position: string;
  gender: string;
  startDate: string;
  isActive: boolean;
  status: EmployeeStatus;
};

export type WithInitials = {
  initials: string;
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
};
