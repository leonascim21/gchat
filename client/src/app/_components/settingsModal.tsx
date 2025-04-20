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
import { Dice5, Settings } from "lucide-react";
import { User } from "../fetchData";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

interface Props {
  user: User;
}

export default function SettingsModal({ user }: Props) {
  const [isEditEmail, setIsEditEmail] = useState(false);
  const [isEditUsername, setIsEditUsername] = useState(false);
  const [isEditPassword, setIsEditPassword] = useState(false);

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
        <div className="flex flex-col w-full gap-4">
          <div className="flex justify-center">
            <div className="relative w-20 h-20">
              <Avatar className="w-full h-full">
                <AvatarImage src={user.profile_picture} />
                <AvatarFallback>
                  {user.username.substring(0, 2).toUpperCase() ?? ""}
                </AvatarFallback>
              </Avatar>
              <Dice5 className="absolute bottom-0 right-0 w-7 h-7 hover:cursor-pointer bg-white rounded-full translate-x-1/3 translate-y-1/3 p-1 text-primary" />
            </div>
          </div>
          <div className="flex flex-col gap-1">
            <p>Email: </p>
            <Input type="email" required={true} value={user.email} />
          </div>
          <div className="flex flex-col gap-1">
            <p>Username: </p>
            <Input type="text" required={true} value={user.username} />
          </div>
          <div className="flex flex-col gap-1">
            <p className="text-nowrap">Current Password: </p>
            <Input type="password" required={true} placeholder="●●●●●●●●" />
          </div>
          <div className="flex flex-col gap-1">
            <p className="text-nowrap">New Password: </p>
            <Input type="password" required={true} placeholder="●●●●●●●●" />
          </div>
          <Button className="max-w-32">Save Changes</Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
