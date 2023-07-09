import { z } from "zod";

export const newPersonInputSchema = z.object({
  name: z.string().min(1),
  nip: z.string().optional(),
  country: z.string().optional(),
  city: z.string().optional(),
  address: z.string().optional(),
  zipCode: z.string().optional(),
  email: z.string().email().optional(),
  phone: z.string().optional(),
  birthday: z.date().nullish().default(null),
  organization: z.string().optional(),
  role: z.string().optional(),
  department: z.string().optional(),
  joiningDate: z.date().default(new Date()).optional(),
});

export type NewPersonInput = z.infer<typeof newPersonInputSchema>;
