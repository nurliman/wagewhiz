export const validateEmail = (email?: string): boolean => {
  // If the email is not provided, return false
  if (!email) return false;

  // Email regular expression pattern
  const emailRegex = /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/;

  // Check if the email matches the pattern
  return emailRegex.test(email);
};
