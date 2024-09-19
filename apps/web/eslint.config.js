import eslintConfig from "@wagewhiz/eslint-config";

/** @type {import('eslint').Linter.Config[]} */
export default [
  ...eslintConfig,
  {
    ignores: ["src/lib/graphql/**/*", "!src/lib/graphql/execute.*"],
  },
];
