'use client";';
import { Input } from "@/components/ui/input";
import {
  SidebarContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from "@/components/ui/sidebar";
import { Group } from "../fetchData";
import { useState } from "react";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Users } from "lucide-react";

interface Props {
  groups: Group[];
  changeChat: (groupId: number) => void;
}

export default function ChatList({ groups, changeChat }: Props) {
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedChat, setSelectedChat] = useState<number | null>(null);
  const { setOpenMobile } = useSidebar();

  return (
    <>
      <div className="px-3 py-1 mb-2">
        <Input
          placeholder="Search chats..."
          className="pl-3 pr-3 py-2 bg-slate-100 dark:bg-slate-800"
          onChange={(e) => setSearchQuery(e.target.value)}
        />
      </div>
      <SidebarContent>
        <SidebarMenu>
          {groups
            .filter((chat) =>
              chat.name.toLowerCase().includes(searchQuery.toLowerCase())
            )
            .map((chat) => (
              <div key={chat.id}>
                <hr className="pb-1 mx-2" />
                <SidebarMenuItem>
                  <SidebarMenuButton
                    onClick={() => {
                      setSelectedChat(chat.id);
                      changeChat(chat.id);
                      setOpenMobile(false);
                    }}
                    isActive={selectedChat === chat.id}
                  >
                    <Avatar className="h-6 w-6">
                      <AvatarImage src={chat.profile_picture} />
                      <AvatarFallback>
                        <div className="bg-purple-200 dark:bg-purple-900 rounded-full h-6 w-6 flex items-center justify-center border border-gray-400">
                          <Users className="h-3 w-3 " />
                        </div>
                      </AvatarFallback>
                    </Avatar>
                    {chat.name}
                  </SidebarMenuButton>
                </SidebarMenuItem>
              </div>
            ))}
        </SidebarMenu>
      </SidebarContent>
    </>
  );
}
