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
import { Circle, Settings } from "lucide-react";
import { Label } from "@/components/ui/label";
import { useState } from "react";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import ThemeToggle from "./theme-toggle";

export default function SettingsModal() {
  const [theme, setTheme] = useState("theme-purple");

  const changeTheme = (newTheme: any) => {
    const classList = document.documentElement.classList;
    classList.remove(
      "theme-violet",
      "theme-blue",
      "theme-green",
      "theme-orange",
      "theme-red"
    );
    classList.add(newTheme);
    setTheme(newTheme);
    console.log(theme);
    localStorage.setItem("theme", newTheme);
  };

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
          Manage your account information and preferences.
        </DialogDescription>
        <div className="flex items-center justify-center">
          <div className="flex flex-row gap-2 items-center">
            <Label htmlFor="theme">Theme</Label>
            <Select
              name="theme"
              onValueChange={(value) => changeTheme(`theme-${value}`)}
              defaultValue={localStorage.getItem("theme")?.slice(6) ?? "violet"}
            >
              <SelectTrigger className="w-[180px]">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectItem value="violet">
                    <div className="flex flex-row gap-1 items-center">
                      <Circle className="h-4 w-4 mr-2 text-violet-500 fill-violet-500" />
                      <p>Violet</p>
                    </div>
                  </SelectItem>
                  <SelectItem value="blue">
                    <div className="flex flex-row gap-1 items-center">
                      <Circle className="h-4 w-4 mr-2 text-blue-500 fill-blue-500" />
                      <p>Blue</p>
                    </div>
                  </SelectItem>
                  <SelectItem value="green">
                    <div className="flex flex-row gap-1 items-center">
                      <Circle className="h-4 w-4 mr-2 text-green-500 fill-green-500" />
                      <p>Green</p>
                    </div>
                  </SelectItem>
                  <SelectItem value="red">
                    <div className="flex flex-row gap-1 items-center">
                      <Circle className="h-4 w-4 mr-2 text-red-500 fill-red-500" />
                      <p>Red</p>
                    </div>
                  </SelectItem>
                  <SelectItem value="orange">
                    <div className="flex flex-row gap-1 items-center">
                      <Circle className="h-4 w-4 mr-2 text-orange-500 fill-orange-500" />
                      <p>Orange</p>
                    </div>
                  </SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
            <ThemeToggle />
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
