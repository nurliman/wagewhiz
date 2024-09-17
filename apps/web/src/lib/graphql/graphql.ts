/* eslint-disable */
import type { DocumentTypeDecoration } from "@graphql-typed-document-node/core";
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = {
  [_ in K]?: never;
};
export type Incremental<T> =
  | T
  | { [P in keyof T]?: P extends " $fragmentName" | "__typename" ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string };
  String: { input: string; output: string };
  Boolean: { input: boolean; output: boolean };
  Int: { input: number; output: number };
  Float: { input: number; output: number };
  /**
   * Implement the DateTime<FixedOffset> scalar
   *
   * The input/output is a string in RFC3339 format.
   */
  DateTime: { input: any; output: any };
  /**
   * ISO 8601 calendar date without timezone.
   * Format: %Y-%m-%d
   *
   * # Examples
   *
   * * `1994-11-13`
   * * `2000-02-24`
   */
  NaiveDate: { input: any; output: any };
  /**
   * A UUID is a unique 128-bit number, stored as 16 octets. UUIDs are parsed as
   * Strings within GraphQL. UUIDs are used to assign unique identifiers to
   * entities without requiring a central allocating authority.
   *
   * # References
   *
   * * [Wikipedia: Universally Unique Identifier](http://en.wikipedia.org/wiki/Universally_unique_identifier)
   * * [RFC4122: A Universally Unique IDentifier (UUID) URN Namespace](http://tools.ietf.org/html/rfc4122)
   */
  UUID: { input: any; output: any };
};

export type AuthResponse = {
  __typename?: "AuthResponse";
  credential: Credential;
  user: UserAccount;
};

export type Credential = {
  __typename?: "Credential";
  accessToken: Scalars["String"]["output"];
  refreshToken: Scalars["String"]["output"];
};

export type LoginInput = {
  password: Scalars["String"]["input"];
  username: Scalars["String"]["input"];
};

export type Mutation = {
  __typename?: "Mutation";
  login: AuthResponse;
  logout: Scalars["Boolean"]["output"];
  refreshToken: AuthResponse;
};

export type MutationLoginArgs = {
  input: LoginInput;
};

export type PeoplePaginationResponse = {
  __typename?: "PeoplePaginationResponse";
  items: Array<Person>;
  page: Scalars["Int"]["output"];
  pageSize: Scalars["Int"]["output"];
  total: Scalars["Int"]["output"];
};

export type Person = {
  __typename?: "Person";
  address?: Maybe<Scalars["String"]["output"]>;
  avatarUrl?: Maybe<Scalars["String"]["output"]>;
  birthday?: Maybe<Scalars["NaiveDate"]["output"]>;
  city?: Maybe<Scalars["String"]["output"]>;
  country?: Maybe<Scalars["String"]["output"]>;
  createdAt: Scalars["DateTime"]["output"];
  deletedAt?: Maybe<Scalars["DateTime"]["output"]>;
  department?: Maybe<Scalars["String"]["output"]>;
  email?: Maybe<Scalars["String"]["output"]>;
  gender?: Maybe<Scalars["String"]["output"]>;
  id: Scalars["UUID"]["output"];
  isActive?: Maybe<Scalars["Boolean"]["output"]>;
  joiningDate?: Maybe<Scalars["NaiveDate"]["output"]>;
  name?: Maybe<Scalars["String"]["output"]>;
  nik?: Maybe<Scalars["String"]["output"]>;
  organization?: Maybe<Scalars["String"]["output"]>;
  phone?: Maybe<Scalars["String"]["output"]>;
  role?: Maybe<Scalars["String"]["output"]>;
  status?: Maybe<Scalars["String"]["output"]>;
  updatedAt: Scalars["DateTime"]["output"];
  zipCode?: Maybe<Scalars["String"]["output"]>;
};

export type Query = {
  __typename?: "Query";
  me: UserAccount;
  people: PeoplePaginationResponse;
  person: Person;
  user: UserAccount;
};

export type QueryPeopleArgs = {
  page?: InputMaybe<Scalars["Int"]["input"]>;
  pageSize?: InputMaybe<Scalars["Int"]["input"]>;
};

export type QueryPersonArgs = {
  id: Scalars["UUID"]["input"];
};

export type QueryUserArgs = {
  id: Scalars["UUID"]["input"];
};

export type UserAccount = {
  __typename?: "UserAccount";
  createdAt: Scalars["DateTime"]["output"];
  deletedAt?: Maybe<Scalars["DateTime"]["output"]>;
  id: Scalars["UUID"]["output"];
  personId?: Maybe<Scalars["UUID"]["output"]>;
  role: Scalars["String"]["output"];
  updatedAt: Scalars["DateTime"]["output"];
  username: Scalars["String"]["output"];
};

export type LoginMutationVariables = Exact<{
  input: LoginInput;
}>;

export type LoginMutation = {
  __typename?: "Mutation";
  login: {
    __typename?: "AuthResponse";
    user: {
      __typename?: "UserAccount";
      id: any;
      createdAt: any;
      updatedAt: any;
      username: string;
      role: string;
      personId?: any | null;
    };
    credential: { __typename?: "Credential"; accessToken: string; refreshToken: string };
  };
};

export type LogoutMutationVariables = Exact<{ [key: string]: never }>;

export type LogoutMutation = { __typename?: "Mutation"; logout: boolean };

export type RefreshTokenMutationVariables = Exact<{ [key: string]: never }>;

export type RefreshTokenMutation = {
  __typename?: "Mutation";
  refreshToken: {
    __typename?: "AuthResponse";
    user: {
      __typename?: "UserAccount";
      id: any;
      createdAt: any;
      updatedAt: any;
      username: string;
      role: string;
      personId?: any | null;
    };
    credential: { __typename?: "Credential"; accessToken: string; refreshToken: string };
  };
};

export type GetMeQueryVariables = Exact<{ [key: string]: never }>;

export type GetMeQuery = {
  __typename?: "Query";
  me: {
    __typename?: "UserAccount";
    id: any;
    createdAt: any;
    updatedAt: any;
    username: string;
    role: string;
    personId?: any | null;
  };
};

export type GetPeopleQueryVariables = Exact<{
  page?: InputMaybe<Scalars["Int"]["input"]>;
  pageSize?: InputMaybe<Scalars["Int"]["input"]>;
}>;

export type GetPeopleQuery = {
  __typename?: "Query";
  people: {
    __typename?: "PeoplePaginationResponse";
    page: number;
    pageSize: number;
    total: number;
    items: Array<{
      __typename?: "Person";
      id: any;
      createdAt: any;
      updatedAt: any;
      name?: string | null;
      nik?: string | null;
      country?: string | null;
      city?: string | null;
      address?: string | null;
      zipCode?: string | null;
      email?: string | null;
      phone?: string | null;
      birthday?: any | null;
      organization?: string | null;
      role?: string | null;
      department?: string | null;
      joiningDate?: any | null;
      isActive?: boolean | null;
      gender?: string | null;
      status?: string | null;
      avatarUrl?: string | null;
    }>;
  };
};

export type GetPersonByIdQueryVariables = Exact<{
  id: Scalars["UUID"]["input"];
}>;

export type GetPersonByIdQuery = {
  __typename?: "Query";
  person: {
    __typename?: "Person";
    id: any;
    createdAt: any;
    updatedAt: any;
    name?: string | null;
    nik?: string | null;
    country?: string | null;
    city?: string | null;
    address?: string | null;
    zipCode?: string | null;
    email?: string | null;
    phone?: string | null;
    birthday?: any | null;
    organization?: string | null;
    role?: string | null;
    department?: string | null;
    joiningDate?: any | null;
    isActive?: boolean | null;
    gender?: string | null;
    status?: string | null;
    avatarUrl?: string | null;
  };
};

export class TypedDocumentString<TResult, TVariables>
  extends String
  implements DocumentTypeDecoration<TResult, TVariables>
{
  __apiType?: DocumentTypeDecoration<TResult, TVariables>["__apiType"];

  constructor(
    private value: string,
    public __meta__?: Record<string, any>,
  ) {
    super(value);
  }

  toString(): string & DocumentTypeDecoration<TResult, TVariables> {
    return this.value;
  }
}

export const LoginDocument = new TypedDocumentString(`
    mutation Login($input: LoginInput!) {
  login(input: $input) {
    user {
      id
      createdAt
      updatedAt
      username
      role
      personId
    }
    credential {
      accessToken
      refreshToken
    }
  }
}
    `) as unknown as TypedDocumentString<LoginMutation, LoginMutationVariables>;
export const LogoutDocument = new TypedDocumentString(`
    mutation Logout {
  logout
}
    `) as unknown as TypedDocumentString<LogoutMutation, LogoutMutationVariables>;
export const RefreshTokenDocument = new TypedDocumentString(`
    mutation RefreshToken {
  refreshToken {
    user {
      id
      createdAt
      updatedAt
      username
      role
      personId
    }
    credential {
      accessToken
      refreshToken
    }
  }
}
    `) as unknown as TypedDocumentString<RefreshTokenMutation, RefreshTokenMutationVariables>;
export const GetMeDocument = new TypedDocumentString(`
    query GetMe {
  me {
    id
    createdAt
    updatedAt
    username
    role
    personId
  }
}
    `) as unknown as TypedDocumentString<GetMeQuery, GetMeQueryVariables>;
export const GetPeopleDocument = new TypedDocumentString(`
    query GetPeople($page: Int, $pageSize: Int) {
  people(page: $page, pageSize: $pageSize) {
    items {
      id
      createdAt
      updatedAt
      name
      nik
      country
      city
      address
      zipCode
      email
      phone
      birthday
      organization
      role
      department
      joiningDate
      isActive
      gender
      status
      avatarUrl
    }
    page
    pageSize
    total
  }
}
    `) as unknown as TypedDocumentString<GetPeopleQuery, GetPeopleQueryVariables>;
export const GetPersonByIdDocument = new TypedDocumentString(`
    query GetPersonById($id: UUID!) {
  person(id: $id) {
    id
    createdAt
    updatedAt
    name
    nik
    country
    city
    address
    zipCode
    email
    phone
    birthday
    organization
    role
    department
    joiningDate
    isActive
    gender
    status
    avatarUrl
  }
}
    `) as unknown as TypedDocumentString<GetPersonByIdQuery, GetPersonByIdQueryVariables>;
