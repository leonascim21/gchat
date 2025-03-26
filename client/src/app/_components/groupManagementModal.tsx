"use client";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { Label } from "@/components/ui/label";
import {
  Dialog,
  DialogContent,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  SidebarHeader,
  SidebarMenu,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import { Users, X } from "lucide-react";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { GroupSettingsForm } from "./groupSettingsForm";
import type { Friend, Group } from "../fetchData";

interface Props {
  group: Group;
  friends: Friend[];
}

export default function GroupManagementModal({ group, friends }: Props) {
  return (
    <Dialog>
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <DialogTrigger asChild>
              <div className="w-full items-center hover:cursor-pointer">
                <div className="flex flex-row gap-2 text-center items-center">
                  <Avatar>
                    <AvatarImage src={group.profile_picture} />
                    <AvatarFallback>
                      <div className="bg-purple-200 dark:bg-purple-900 rounded-full h-10 w-10 flex items-center justify-center border border-gray-400">
                        <Users className="h-5 w-5 " />
                      </div>
                    </AvatarFallback>
                  </Avatar>
                  <p className="hover:text-gray-600 dark:hover:text-gray-400">
                    {group.name}
                  </p>
                </div>
              </div>
            </DialogTrigger>
            <DialogContent>
              <DialogTitle className="text-center">
                <div className="flex flex-row gap-3 items-center text-center justify-center">
                  <Avatar>
                    <AvatarImage src={group.profile_picture} />
                    <AvatarFallback>
                      <div className="bg-purple-200 dark:bg-purple-900 rounded-full h-10 w-10 flex items-center justify-center border border-gray-400">
                        <Users className="h-5 w-5 " />
                      </div>
                    </AvatarFallback>
                  </Avatar>
                  <h1>{group.name}</h1>
                </div>
                <p className="font-normal text-sm">
                  {group.members.length} members
                </p>
              </DialogTitle>
              <ScrollArea className="h-[215px] pr-4 pt-5">
                <div className="flex flex-col gap-2">
                  {group.members.map((friend) => (
                    <div
                      key={friend.friend_id}
                      className="space-y-1 flex flex-row gap-3 pb-2 items-center border-b justify-between"
                    >
                      <div className="flex flex-row gap-2 items-center">
                        <Avatar className="h-7 w-7">
                          <AvatarImage src={friend.profile_picture} />
                          <AvatarFallback>
                            {friend?.username.substring(0, 2).toUpperCase() ??
                              ""}
                          </AvatarFallback>
                        </Avatar>
                        <Label>{friend.username}</Label>
                      </div>
                      <div className="flex flex-row gap-3">
                        <TooltipProvider delayDuration={0}>
                          <Tooltip>
                            <TooltipTrigger asChild>
                              <X className="h-4 w-4 font-semibold hover:text-primary hover:cursor-pointer" />
                            </TooltipTrigger>
                            <TooltipContent>
                              <p>Remove from group</p>
                            </TooltipContent>
                          </Tooltip>
                        </TooltipProvider>
                      </div>
                    </div>
                  ))}
                </div>
              </ScrollArea>
              <GroupSettingsForm
                key={group.id}
                friends={friends.filter(
                  (friend) =>
                    !group.members.some(
                      (member) => member.friend_id === friend.friend_id
                    )
                )}
                groupId={group.id}
              />
            </DialogContent>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
    </Dialog>
  );
}
