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
import { users } from "../fakeData/fakeData";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { generateProfilePictureSVG } from "../utils";
import { GroupSettingsForm } from "./groupSettingsForm";

interface Props {
  groupName: string;
  groupId: number;
  profilePicture: string;
}

const friends = users;
const friendsInGroup = users.slice(0, 10);

export default function GroupManagementModal({
  groupId,
  groupName,
  profilePicture,
}: Props) {
  return (
    <Dialog>
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <DialogTrigger asChild>
              <div className="w-full items-center hover:cursor-pointer">
                <div className="flex flex-row gap-2 text-center items-center">
                  <div className="bg-purple-200 dark:bg-purple-900 rounded-full h-8 w-8 flex items-center justify-center border border-gray-400">
                    <Users className="h-4 w-4 " />
                  </div>
                  <p className="hover:text-gray-600 dark:hover:text-gray-400">
                    {groupName} {[groupId]}
                  </p>
                </div>
              </div>
            </DialogTrigger>
            <DialogContent>
              <DialogTitle className="text-center">
                <div className="flex flex-row gap-3 items-center text-center justify-center">
                  <Avatar>
                    <AvatarImage src={profilePicture} />
                    <AvatarFallback>
                      <div className="bg-purple-200 dark:bg-purple-900 rounded-full h-10 w-10 flex items-center justify-center border border-gray-400">
                        <Users className="h-5 w-5 " />
                      </div>
                    </AvatarFallback>
                  </Avatar>

                  <h1>{groupName}</h1>
                </div>
                <p className="font-normal text-sm">{friends.length} members</p>
              </DialogTitle>
              <ScrollArea className="h-[215px] pr-4 pt-5">
                <div className="space-y-2">
                  {friendsInGroup.map((friend) => (
                    <div
                      key={friend.userID}
                      className="space-y-1 flex flex-row gap-3 pb-2 items-center border-b justify-between"
                    >
                      <div className="flex flex-row gap-2 items-center">
                        <Avatar className="h-7 w-7">
                          <AvatarImage
                            src={generateProfilePictureSVG(
                              "username" + `${friend.userID * 23}`
                            )}
                          />
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
                friends={friends.filter(
                  (friend) => !friendsInGroup.includes(friend)
                )}
              />
            </DialogContent>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
    </Dialog>
  );
}
