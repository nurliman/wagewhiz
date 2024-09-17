/* eslint-disable */
import * as types from "./graphql";

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 */
const documents = {
  "\n  mutation Login($input: LoginInput!) {\n    login(input: $input) {\n      user {\n        id\n        createdAt\n        updatedAt\n        username\n        role\n        personId\n      }\n      credential {\n        accessToken\n        refreshToken\n      }\n    }\n  }\n":
    types.LoginDocument,
  "\n  mutation Logout {\n    logout\n  }\n": types.LogoutDocument,
  "\n  mutation RefreshToken {\n    refreshToken {\n      user {\n        id\n        createdAt\n        updatedAt\n        username\n        role\n        personId\n      }\n      credential {\n        accessToken\n        refreshToken\n      }\n    }\n  }\n":
    types.RefreshTokenDocument,
  "\n  query GetMe {\n    me {\n      id\n      createdAt\n      updatedAt\n      username\n      role\n      personId\n    }\n  }\n":
    types.GetMeDocument,
  "\n  query GetPeople($page: Int, $pageSize: Int) {\n    people(page: $page, pageSize: $pageSize) {\n      items {\n        id\n        createdAt\n        updatedAt\n        name\n        nik\n        country\n        city\n        address\n        zipCode\n        email\n        phone\n        birthday\n        organization\n        role\n        department\n        joiningDate\n        isActive\n        gender\n        status\n        avatarUrl\n      }\n      page\n      pageSize\n      total\n    }\n  }\n":
    types.GetPeopleDocument,
  "\n  query GetPersonById($id: UUID!) {\n    person(id: $id) {\n      id\n      createdAt\n      updatedAt\n      name\n      nik\n      country\n      city\n      address\n      zipCode\n      email\n      phone\n      birthday\n      organization\n      role\n      department\n      joiningDate\n      isActive\n      gender\n      status\n      avatarUrl\n    }\n  }\n":
    types.GetPersonByIdDocument,
};

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(
  source: "\n  mutation Login($input: LoginInput!) {\n    login(input: $input) {\n      user {\n        id\n        createdAt\n        updatedAt\n        username\n        role\n        personId\n      }\n      credential {\n        accessToken\n        refreshToken\n      }\n    }\n  }\n",
): typeof import("./graphql").LoginDocument;
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(
  source: "\n  mutation Logout {\n    logout\n  }\n",
): typeof import("./graphql").LogoutDocument;
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(
  source: "\n  mutation RefreshToken {\n    refreshToken {\n      user {\n        id\n        createdAt\n        updatedAt\n        username\n        role\n        personId\n      }\n      credential {\n        accessToken\n        refreshToken\n      }\n    }\n  }\n",
): typeof import("./graphql").RefreshTokenDocument;
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(
  source: "\n  query GetMe {\n    me {\n      id\n      createdAt\n      updatedAt\n      username\n      role\n      personId\n    }\n  }\n",
): typeof import("./graphql").GetMeDocument;
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(
  source: "\n  query GetPeople($page: Int, $pageSize: Int) {\n    people(page: $page, pageSize: $pageSize) {\n      items {\n        id\n        createdAt\n        updatedAt\n        name\n        nik\n        country\n        city\n        address\n        zipCode\n        email\n        phone\n        birthday\n        organization\n        role\n        department\n        joiningDate\n        isActive\n        gender\n        status\n        avatarUrl\n      }\n      page\n      pageSize\n      total\n    }\n  }\n",
): typeof import("./graphql").GetPeopleDocument;
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(
  source: "\n  query GetPersonById($id: UUID!) {\n    person(id: $id) {\n      id\n      createdAt\n      updatedAt\n      name\n      nik\n      country\n      city\n      address\n      zipCode\n      email\n      phone\n      birthday\n      organization\n      role\n      department\n      joiningDate\n      isActive\n      gender\n      status\n      avatarUrl\n    }\n  }\n",
): typeof import("./graphql").GetPersonByIdDocument;

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}
