import { User } from "@clerk/nextjs/server";

export const stripPrivateUserData = (user: User) => {
  const { id, firstName, lastName, imageUrl, publicMetadata, username } = user;

  return {
    id,
    firstName,
    lastName,
    imageUrl,
    publicMetadata,
    username,
  };
};
