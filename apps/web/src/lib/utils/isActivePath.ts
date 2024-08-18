import isRelativeUrl from "is-relative-url";
import startsWith from "lodash-es/startsWith";

export const isDOM = (): boolean => {
  return typeof window !== "undefined" && window.document && !!window.document.documentElement;
};

export const extractPathname = (url: string): string => {
  return isRelativeUrl(url) ? url : new URL(url).pathname;
};

export const getCurrentPath = (): string | undefined => {
  return isDOM() ? window.location.pathname : undefined;
};

export const isActivePath = (path: string, currentPath = getCurrentPath()): boolean => {
  if (!path || !currentPath) return false;

  const normalizedPath = extractPathname(path);
  const normalizedCurrentPath = extractPathname(currentPath);

  // Check if the currentPath starts with the path and is followed by a '/' or end of string
  return (
    startsWith(normalizedCurrentPath, normalizedPath) &&
    (normalizedCurrentPath[normalizedPath.length] === "/" ||
      normalizedCurrentPath.length === normalizedPath.length)
  );
};
