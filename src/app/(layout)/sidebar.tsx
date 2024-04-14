"use client";

import Link from "next/link";
import { Home, Settings, LibraryBigIcon, UsersIcon } from "lucide-react";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { cn } from "@/lib/utils";
import { usePathname } from "next/navigation";

export const Sidebar = () => {
  return (
    <aside className="fixed inset-y-0 left-0 z-10 hidden w-14 flex-col border-r bg-background sm:flex">
      <nav className="flex flex-col items-center gap-4 px-2 sm:py-4">
        <Link
          href="#"
          className="group flex h-9 w-9 shrink-0 items-center justify-center gap-2 rounded-full bg-primary text-lg font-semibold text-primary-foreground md:h-8 md:w-8 md:text-base"
        >
          <LibraryBigIcon className="h-4 w-4 transition-all group-hover:scale-110" />
          <span className="sr-only">Teacher Tools</span>
        </Link>
        <SidebarLink
          icon={<Home className="h-5 w-5" />}
          label="Dashboard"
          href="dashboard"
        />
        <SidebarLink
          icon={<UsersIcon className="h-5 w-5" />}
          label="Groups"
          href="groups"
        />
      </nav>
      <nav className="mt-auto flex flex-col items-center gap-4 px-2 sm:py-4">
        <Tooltip>
          <TooltipTrigger asChild>
            <Link
              href="/settings"
              className="flex h-9 w-9 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:text-foreground md:h-8 md:w-8"
            >
              <Settings className="h-5 w-5" />
              <span className="sr-only">Settings</span>
            </Link>
          </TooltipTrigger>
          <TooltipContent side="right">Settings</TooltipContent>
        </Tooltip>
      </nav>
    </aside>
  );
};

type SidebarLinkProps = {
  icon: React.ReactNode;
  label: string;
  href: string;
};

const SidebarLink = (props: SidebarLinkProps) => {
  const pathname = usePathname().split("/").filter(Boolean);

  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <Link
          href={`/${props.href}`}
          className={cn(
            "flex h-9 w-9 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:text-foreground md:h-8 md:w-8",
            pathname.length > 0
              ? pathname.at(0)?.toLowerCase() === props.href && "bg-accent"
              : props.href === "dashboard" && "bg-accent"
          )}
        >
          {props.icon}
          <span className="sr-only">{props.label}</span>
        </Link>
      </TooltipTrigger>
      <TooltipContent side="right">{props.label}</TooltipContent>
    </Tooltip>
  );
};
