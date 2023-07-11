export const padNumberWithZeros = (number: number, length: number): string => {
  if (typeof number !== "number" || typeof length !== "number" || number <= 0 || length <= 0) {
    throw new Error("Invalid input. Both number and length must be positive numbers.");
  }

  let str = "" + number;

  while (str.length < length) {
    str = "0" + str;
  }

  return str;
};
