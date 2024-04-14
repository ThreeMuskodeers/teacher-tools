import { initTRPC, TRPCError } from "@trpc/server";
import superjson from "superjson";
import { ZodError } from "zod";
import {
  auth,
  SignedInAuthObject,
  SignedOutAuthObject,
} from "@clerk/nextjs/server";
import { db } from "@/server/db";

interface AuthContext {
  auth: SignedInAuthObject | SignedOutAuthObject;
}

export const createAuthContextInner = async ({ auth }: AuthContext) => {
  return {
    auth,
    db,
  };
};

export const createTRPCContext = async (opts: { headers: Headers }) => {
  return createAuthContextInner({
    auth: auth(),
    ...opts,
  });
};

const t = initTRPC.context<typeof createTRPCContext>().create({
  transformer: superjson,
  errorFormatter({ shape, error }) {
    return {
      ...shape,
      data: {
        ...shape.data,
        zodError:
          error.cause instanceof ZodError ? error.cause.flatten() : null,
      },
    };
  },
});
const isAuthed = t.middleware(({ next, ctx }) => {
  if (!ctx.auth.userId) {
    throw new TRPCError({
      code: "UNAUTHORIZED",
      message: "You must be logged in to access this API.",
    });
  }
  return next({
    ctx: {
      auth: ctx.auth,
    },
  });
});

export const createCallerFactory = t.createCallerFactory;
export const createTRPCRouter = t.router;

export const publicProcedure = t.procedure;
export const protectedProcedure = t.procedure.use(isAuthed);
