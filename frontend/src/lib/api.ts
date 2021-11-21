import { variables } from "$lib/env";
const base = variables.apiUrl;

export async function query(
  fetch: (info: RequestInfo, init?: RequestInit) => Promise<Response>,
  query: string
) {
  return await fetch(`${base}/graphql`, {
    method: "POST",
    mode: "cors",
    headers: {
      "content-type": "application/json",
    },
    body: JSON.stringify({
      query,
    }),
  });
}

export async function createUser({name, email}) {
  return await fetch(`${base}/graphql`, {
    method: "POST",
    headers: {
      "content-type": "application/json",
    },
    body: JSON.stringify({
      query: `mutation create_user($name: String!, $email: String!) {
        createUser(name: $name, email: $email) { id, name, email}
      }`,
      variables: {
        name,
        email
      },
    }),
  });
}

export async function createTransaction({fromId, toId, amount, note}) {
  return await fetch(`${base}/graphql`, {
    method: "POST",
    headers: {
      "content-type": "application/json",
    },
    body: JSON.stringify({
      query: `mutation create_transaction($fromId: String!, $toId: String!, $amount: Int!, $note: String) {
        createTransaction(fromId: $fromId, toId: $toId, amount: $amount, note: $note) { id, fromId, toId, amount, note, insertedAt }
      }`,
      variables: {
        fromId,
        toId,
        amount,
        note
      },
    }),
  });
}

