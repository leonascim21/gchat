'use client";';
import { Input } from "@/components/ui/input";
import {
  SidebarContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from "@/components/ui/sidebar";
import { Group, User, TempGroup } from "../fetchData";
import { useEffect, useState } from "react";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Hourglass, Users } from "lucide-react";
import Link from "next/link";
import CopyToClipboardButton from "./clipboardButton";

interface Props {
  groups: Group[];
  tempGroups: TempGroup[];
  user: User | null;
  changeChat: (groupId: number) => void;
}

export default function ChatList({
  groups,
  changeChat,
  user,
  tempGroups,
}: Props) {
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedChat, setSelectedChat] = useState<number | null>(null);
  const { setOpenMobile } = useSidebar();
  const [recentChats, setRecentChats] = useState<Group[]>([]);

  useEffect(() => {
    const storedRecentChats = localStorage.getItem("recentChats");
    if (storedRecentChats) {
      const parsedRecentChats = JSON.parse(storedRecentChats);
      setRecentChats(parsedRecentChats);
    }
  }, []);

  const addRecentChat = (group: Group) => {
    const updatedRecentChats = [
      group,
      ...recentChats.filter((chat) => chat.id !== group.id),
    ].slice(0, 5);
    setRecentChats(updatedRecentChats);
    localStorage.setItem("recentChats", JSON.stringify(updatedRecentChats));
  };

  const groupChats = groups.filter((chat) => chat.group_type === 1);
  const privateChats = groups.filter((chat) => chat.group_type === 2);

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
          {recentChats.filter((chat) =>
            chat.name.toLowerCase().includes(searchQuery.toLowerCase())
          ).length > 0 && (
            <h1 className="text-sm px-3 pt-3 font-semibold">Recent Chats</h1>
          )}
          {recentChats
            .filter((chat) =>
              chat.name.toLowerCase().includes(searchQuery.toLowerCase())
            )
            .map((chat) =>
              chat.group_type === 1 ? (
                <div key={chat.id}>
                  <hr className="pb-1 mx-2" />
                  <SidebarMenuItem>
                    <SidebarMenuButton
                      onClick={() => {
                        setSelectedChat(chat.id);
                        changeChat(chat.id);
                        addRecentChat(chat);
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
              ) : (
                <div key={chat.id}>
                  <hr className="pb-1 mx-2" />
                  <SidebarMenuItem>
                    <SidebarMenuButton
                      onClick={() => {
                        setSelectedChat(chat.id);
                        changeChat(chat.id);
                        addRecentChat(chat);
                        setOpenMobile(false);
                      }}
                      isActive={selectedChat === chat.id}
                    >
                      <Avatar className="h-6 w-6">
                        <AvatarImage
                          src={
                            chat.members.filter(
                              (member) => member.friend_id !== (user?.id ?? 0)
                            )[0]?.profile_picture
                          }
                        />
                        <AvatarFallback>
                          <div className="bg-purple-200 dark:bg-purple-900 rounded-full h-6 w-6 flex items-center justify-center border border-gray-400">
                            {chat.members
                              .filter(
                                (member) => member.friend_id !== (user?.id ?? 0)
                              )[0]
                              ?.username.substring(0, 2)
                              .toUpperCase() ?? ""}
                          </div>
                        </AvatarFallback>
                      </Avatar>
                      {
                        chat.members.filter(
                          (member) => member.friend_id !== (user?.id ?? 0)
                        )[0]?.username
                      }
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                </div>
              )
            )}
          {groupChats.filter((chat) =>
            chat.name.toLowerCase().includes(searchQuery.toLowerCase())
          ).length > 0 && (
            <h1 className="text-sm px-3 pt-3 font-semibold">Group Chats</h1>
          )}
          {groupChats
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
                      addRecentChat(chat);
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
          {privateChats.filter((chat) =>
            chat.name.toLowerCase().includes(searchQuery.toLowerCase())
          ).length > 0 && (
            <h1 className="text-sm px-3 pt-3 font-semibold">DM&#39;s</h1>
          )}
          {privateChats
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
                      addRecentChat(chat);
                      setOpenMobile(false);
                    }}
                    isActive={selectedChat === chat.id}
                  >
                    <Avatar className="h-6 w-6">
                      <AvatarImage
                        src={
                          chat.members.filter(
                            (member) => member.friend_id !== (user?.id ?? 0)
                          )[0]?.profile_picture
                        }
                      />
                      <AvatarFallback>
                        <div className="bg-purple-200 dark:bg-purple-900 rounded-full h-6 w-6 flex items-center justify-center border border-gray-400">
                          {chat.members
                            .filter(
                              (member) => member.friend_id !== (user?.id ?? 0)
                            )[0]
                            ?.username.substring(0, 2)
                            .toUpperCase() ?? ""}
                        </div>
                      </AvatarFallback>
                    </Avatar>
                    {
                      chat.members.filter(
                        (member) => member.friend_id !== (user?.id ?? 0)
                      )[0]?.username
                    }
                  </SidebarMenuButton>
                </SidebarMenuItem>
              </div>
            ))}
          {tempGroups.filter((chat) =>
            chat.name.toLowerCase().includes(searchQuery.toLowerCase())
          ).length > 0 && (
            <h1 className="text-sm px-3 pt-3 font-semibold">Temporary Chats</h1>
          )}
          {tempGroups
            .filter((chat) =>
              chat.name.toLowerCase().includes(searchQuery.toLowerCase())
            )
            .map((chat) => (
              <div key={chat.id}>
                <hr className="pb-1 mx-2" />
                <SidebarMenuItem>
                  <SidebarMenuButton
                    className="h-10"
                    isActive={selectedChat === chat.id}
                  >
                    <div className="flex flex-row justify-between items-center w-full pr-2">
                      <Link href={`/${chat.temp_chat_key}`}>
                        <div className="bg-purple-200 dark:bg-purple-900 rounded-full h-6 w-6 flex items-center justify-center border border-gray-400">
                          <Hourglass className="h-3 w-3" />
                        </div>
                      </Link>
                      <Link
                        href={`/${chat.temp_chat_key}`}
                        className="flex flex-col text-center"
                      >
                        <p>{chat.name}</p>
                        <p className="text-sm">
                          {new Date(chat.end_date).toLocaleDateString([], {
                            month: "short",
                            day: "numeric",
                          }) +
                            " • " +
                            new Date(chat.end_date).toLocaleTimeString([], {
                              hour: "2-digit",
                              minute: "2-digit",
                            })}
                        </p>
                      </Link>
                      <CopyToClipboardButton
                        text={`https://gchat.cloud/${chat.temp_chat_key}`}
                      />
                    </div>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              </div>
            ))}
        </SidebarMenu>
      </SidebarContent>
    </>
  );
}
