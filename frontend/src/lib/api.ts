export async function createUser({name, email}) {
  return await fetch(`http://localhost:5050/graphql`, {
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
