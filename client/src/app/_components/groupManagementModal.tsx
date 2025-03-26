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
import { useEffect, useState } from "react";
import axios from "axios";

interface Friend {
  friend_id: number;
  username: string;
  profile_picture: string;
}

interface Props {
  groupName: string;
  groupId: number;
  profilePicture: string;
  friends: Friend[];
}

export default function GroupManagementModal({
  groupId,
  groupName,
  profilePicture,
  friends,
}: Props) {
  const [groupMembers, setGroupMembers] = useState<Friend[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(true);

  useEffect(() => {
    const token = localStorage.getItem("token");
    if (!token) return;

    axios
      .get(
        `http://localhost:3001/group/get-users?token=${token}&group_id=${groupId}`
      )
      .then((response) => {
        setGroupMembers(response.data);
      })
      .catch((error) => {
        console.error(error);
      })
      .finally(() => {
        setIsLoading(false);
      });
  }, [groupId]);
  return (
    <Dialog>
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <DialogTrigger asChild>
              <div className="w-full items-center hover:cursor-pointer">
                <div className="flex flex-row gap-2 text-center items-center">
                  <Avatar>
                    <AvatarImage src={profilePicture} />
                    <AvatarFallback>
                      <div className="bg-purple-200 dark:bg-purple-900 rounded-full h-10 w-10 flex items-center justify-center border border-gray-400">
                        <Users className="h-5 w-5 " />
                      </div>
                    </AvatarFallback>
                  </Avatar>
                  <p className="hover:text-gray-600 dark:hover:text-gray-400">
                    {groupName}
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
                <p className="font-normal text-sm">
                  {groupMembers.length} members
                </p>
              </DialogTitle>
              <ScrollArea className="h-[215px] pr-4 pt-5">
                <div className="space-y-2">
                  {groupMembers.map((friend) => (
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
              {!isLoading && (
                <GroupSettingsForm
                  key={groupId}
                  friends={friends.filter(
                    (friend) => !groupMembers.includes(friend)
                  )}
                  groupId={groupId}
                />
              )}
            </DialogContent>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
    </Dialog>
  );
}
