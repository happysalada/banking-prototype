export async function createUser({name, email}) {
  return await fetch(`localhost:5050/graphql`, {
    method: "POST",
    headers: {
      "content-type": "application/json",
    },
    body: JSON.stringify({
      query: `mutation create_user($user: NewUser!) {
        createUser(newUser: $user) { id, name, email}
      }`,
      variables: {
        user: {
          name,
          email
        },
      },
    }),
  });
}
