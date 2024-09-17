import type { CodegenConfig } from "@graphql-codegen/cli";

const config: CodegenConfig = {
  schema: "http://localhost:3001/graphql",
  documents: ["src/lib/**/*.ts"],
  generates: {
    "src/lib/graphql/": {
      preset: "client",
      config: {
        documentMode: "string",
        useTypeImports: true,
      },
    },
    "schema.graphql": {
      plugins: ["schema-ast"],
      config: {
        includeDirectives: true,
      },
    },
  },
};

export default config;
