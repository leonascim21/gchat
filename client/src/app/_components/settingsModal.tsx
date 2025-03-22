"use client";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { Settings } from "lucide-react";

export default function SettingsModal() {
  return (
    <Dialog>
      <DialogTrigger asChild>
        <button className="hover:cursor-pointer">
          <Tooltip>
            <TooltipTrigger asChild>
              <Settings className="h-5 w-5 font-semibold hover:text-primary" />
            </TooltipTrigger>
            <TooltipContent>
              <p>Settings</p>
            </TooltipContent>
          </Tooltip>
        </button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-md">
        <DialogTitle className="text-2xl text-center">Settings</DialogTitle>
        <DialogDescription className="text-center">
          Manage your account information
        </DialogDescription>
        <div className="flex items-center justify-center">Coming Soon</div>
      </DialogContent>
    </Dialog>
  );
}
