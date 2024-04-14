import type { Metadata } from "next";
import { Inter as FontSans } from "next/font/google";
import { ClerkProvider, SignedIn, SignedOut, UserButton } from "@clerk/nextjs";
import { TRPCReactProvider } from "@/trpc/react";
import { RootProviders } from "./providers";
import { cn } from "@/lib/utils";
import { Dashboard } from "./(layout)";
import "./globals.css";

const fontSans = FontSans({
  subsets: ["latin"],
  variable: "--font-sans",
});
export const metadata: Metadata = {
  title: "Teacher Tools",
  description: "",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <TRPCReactProvider>
        <ClerkProvider>
          <body
            className={cn(
              "min-h-screen bg-background font-sans antialiased flex flex-col",
              fontSans.variable
            )}
          >
            <RootProviders>
              <SignedIn>
                <Dashboard>{children}</Dashboard>
              </SignedIn>
              <SignedOut>{children}</SignedOut>
            </RootProviders>
          </body>
        </ClerkProvider>
      </TRPCReactProvider>
    </html>
  );
}
