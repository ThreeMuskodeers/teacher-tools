import { z } from "zod";

import { createTRPCRouter, protectedProcedure } from "@/server/api/trpc";

export const classRouter = createTRPCRouter({
  get: protectedProcedure
    .input(z.object({ id: z.number().int() }))
    .query(({ ctx, input }) => {
      return ctx.db.class.findFirst({
        where: {
          id: input.id,
        },
      });
    }),
  getAll: protectedProcedure.query(({ ctx }) => {
    return ctx.db.class.findMany();
  }),
  create: protectedProcedure
    .input(
      z.object({
        name: z.string(),
        teacherId: z.string(),
      })
    )
    .mutation(({ ctx, input }) => {
      return ctx.db.class.create({
        data: {
          ...input,
          teacherId: ctx.auth.userId,
          creatorId: ctx.auth.userId,
        },
      });
    }),
  update: protectedProcedure
    .input(
      z.object({
        id: z.number().int(),
        name: z.string(),
      })
    )
    .mutation(({ ctx, input }) => {
      return ctx.db.class.update({
        where: {
          id: input.id,
        },
        data: input,
      });
    }),
  delete: protectedProcedure
    .input(z.object({ id: z.number().int() }))
    .mutation(({ ctx, input }) => {
      return ctx.db.class.delete({
        where: {
          id: input.id,
        },
      });
    }),
});
