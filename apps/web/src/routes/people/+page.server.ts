import dayjs from "dayjs";
import initials from "initials";
import { people } from "$lib/data/people.ts";
import type { Employee, WithInitials } from "$lib/types.ts";
import type { PageServerLoad } from "./$types.ts";

export const load = (async () => {
  const peopleWithFormattedDate = people.map((item) => {
    return {
      ...item,
      startDate: dayjs(item.startDate).format("DD MMM YYYY"),
    };
  });

  const peopleWithInitials = peopleWithFormattedDate.map((item) => {
    return {
      ...item,
      initials: initials(item.name),
    } satisfies Employee & WithInitials;
  });

  return {
    people: peopleWithInitials,
    peopleCount: peopleWithInitials.length,
  };
}) satisfies PageServerLoad;
