import { z } from "zod";

import { createTRPCRouter, protectedProcedure } from "@/server/api/trpc";

export const teacherRouter = createTRPCRouter({
  get: protectedProcedure
    .input(z.object({ userId: z.string() }))
    .query(({ ctx, input }) => {
      return ctx.db.teacher.findFirst({
        where: {
          userId: input.userId,
        },
      });
    }),
  getAll: protectedProcedure.query(({ ctx }) => {
    return ctx.db.teacher.findMany();
  }),
  create: protectedProcedure
    .input(
      z.object({
        userId: z.string(),
        school: z.string().optional(),
      })
    )
    .mutation(({ ctx, input }) => {
      return ctx.db.teacher.create({
        data: input,
      });
    }),
  update: protectedProcedure
    .input(
      z.object({
        userId: z.string(),
        school: z.string().optional(),
      })
    )
    .mutation(({ ctx, input }) => {
      return ctx.db.teacher.update({
        where: {
          userId: input.userId,
        },
        data: input,
      });
    }),
  delete: protectedProcedure
    .input(z.object({ userId: z.string() }))
    .mutation(({ ctx, input }) => {
      return ctx.db.teacher.delete({
        where: {
          userId: input.userId,
        },
      });
    }),
});
