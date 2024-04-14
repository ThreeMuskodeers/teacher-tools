import { classRouter } from "@/server/api/routers/class";
import { studentRouter } from "@/server/api/routers/student";
import { teacherRouter } from "@/server/api/routers/teacher";
import { createCallerFactory, createTRPCRouter } from "@/server/api/trpc";

export const appRouter = createTRPCRouter({
  class: classRouter,
  student: studentRouter,
  teacher: teacherRouter,
});

export type AppRouter = typeof appRouter;
export const createCaller = createCallerFactory(appRouter);
