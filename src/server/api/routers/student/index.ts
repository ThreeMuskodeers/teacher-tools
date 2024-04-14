import { z } from "zod";

import { createTRPCRouter, protectedProcedure } from "@/server/api/trpc";

export const studentRouter = createTRPCRouter({
  get: protectedProcedure
    .input(z.object({ id: z.number().int() }))
    .query(({ ctx, input }) => {
      return ctx.db.student.findFirst({
        where: {
          id: input.id,
        },
      });
    }),
  getAll: protectedProcedure.query(({ ctx }) => {
    return ctx.db.student.findMany();
  }),
  create: protectedProcedure
    .input(
      z.object({
        firstName: z.string(),
        lastName: z.string(),
      })
    )
    .mutation(({ ctx, input }) => {
      return ctx.db.student.create({
        data: input,
      });
    }),
  update: protectedProcedure
    .input(
      z.object({
        id: z.number().int(),
        firstName: z.string(),
        lastName: z.string(),
      })
    )
    .mutation(({ ctx, input }) => {
      return ctx.db.student.update({
        where: {
          id: input.id,
        },
        data: input,
      });
    }),
  delete: protectedProcedure
    .input(z.object({ id: z.number().int() }))
    .mutation(({ ctx, input }) => {
      return ctx.db.student.delete({
        where: {
          id: input.id,
        },
      });
    }),
});
