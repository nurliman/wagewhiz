export type EmployeeStatus = "Permanent" | "Contract" | "Probation";

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
