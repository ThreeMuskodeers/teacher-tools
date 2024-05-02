import { z } from "zod";
import { clerkClient } from "@clerk/nextjs";
import { createTRPCRouter, protectedProcedure } from "@/server/api/trpc";
import { stripPrivateUserData } from "./service";

export const teacherRouter = createTRPCRouter({
  get: protectedProcedure
    .input(z.object({ id: z.string() }))
    .query(async ({ ctx }) => {
      return stripPrivateUserData(
        await clerkClient.users.getUser(ctx.auth.userId)
      );
    }),
  getAll: protectedProcedure.query(async () => {
    return (await clerkClient.users.getUserList()).map((user) =>
      stripPrivateUserData(user)
    );
  }),
  create: protectedProcedure
    .input(
      z.object({
        firstName: z.string(),
        lastName: z.string(),
        email: z.string(),
        school: z.string().optional(),
      })
    )
    .mutation(async ({ ctx, input }) => {
      return await clerkClient.users.createUser({
        emailAddress: [input.email],
        firstName: input.firstName,
        lastName: input.lastName,
        publicMetadata: {
          school: input.school,
        },
      });
    }),
  update: protectedProcedure
    .input(
      z
        .object({
          school: z.string(),
        })
        .partial()
    )
    .mutation(async ({ ctx, input }) => {
      return await clerkClient.users.updateUser(ctx.auth.userId, {
        publicMetadata: {
          school: input.school,
        },
      });
    }),
});
