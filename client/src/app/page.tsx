"use client";

import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { LogOut, Menu, MessageSquare, Send, Users } from "lucide-react";
import { useEffect, useState, useRef } from "react";
import Ping from "./_components/ping";
import ThemeToggle from "./_components/theme-toggle";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarInset,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
  SidebarTrigger,
} from "@/components/ui/sidebar";
import CreateChatForm from "./_components/createChatForm";
import ManageFriends from "./_components/manageFriends";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import SettingsModal from "./_components/settingsModal";
import { getIdFromJWT, usernameToColor } from "./utils";
import axios from "axios";
import AuthModals from "./_components/authModals";
import GroupManagementModal from "./_components/groupManagementModal";
import { fetchAll } from "./fetchData";
import type { User, Message, Group, Friend, FriendRequest } from "./fetchData";

export default function Home() {
  const [initialLoad, setInitialLoad] = useState(true);
  const [searchQuery, setSearchQuery] = useState("");
  const [inputMessage, setInputMessage] = useState("");
  const [connected, setConnected] = useState(false);
  const [showAuthModal, setShowAuthModal] = useState(false);
  const [isAuth, setIsAuth] = useState(false);

  const [selectedChat, setSelectedChat] = useState<number | null>(null);
  const [user, setUser] = useState<User | null>(null);
  const [messages, setMessages] = useState<Message[]>([]);
  const [groups, setGroups] = useState<Group[]>([]);
  const [friends, setFriends] = useState<Friend[]>([]);
  const [incomingFriends, setIncomingFriends] = useState<FriendRequest[]>([]);
  const [outgoingFriends, setOutgoingFriends] = useState<FriendRequest[]>([]);

  const wsRef = useRef<WebSocket | null>(null);

  const scrollRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [selectedChat, messages]);

  useEffect(() => {
    const loadInitialData = async () => {
      const token = localStorage.getItem("token");

      if (!token) {
        console.log("Token not found in localStorage.");
        setShowAuthModal(true);
        setIsAuth(false);
        setInitialLoad(false);
        return;
      }

      try {
        const tokenCheckResponse = await axios.get(
          `https://api.gchat.cloud/check-token?token=${token}`
        );

        if (!tokenCheckResponse.data.valid) {
          console.log("Token is invalid.");
          setShowAuthModal(true);
          setIsAuth(false);
          setInitialLoad(false);
          localStorage.removeItem("token");
          return;
        }

        setIsAuth(true);
        const fetchedData = await fetchAll();

        console.log("Data fetched successfully:", fetchedData);

        setUser(fetchedData.user);
        setMessages(fetchedData.messages);
        setGroups(fetchedData.groups);
        setFriends(fetchedData.friends);
        setIncomingFriends(fetchedData.incomingFriends);
        setOutgoingFriends(fetchedData.outgoingFriends);
      } catch (error) {
        console.error("Error during initial data load:", error);
        setShowAuthModal(true);
        setIsAuth(false);
        localStorage.removeItem("token");
      } finally {
        setInitialLoad(false);
      }
    };

    loadInitialData();
  }, []);

  useEffect(() => {
    const token = localStorage.getItem("token");
    if (!token || !isAuth) return;

    const ws = new WebSocket(`wss://ws.gchat.cloud/ws?token=${token}`);
    wsRef.current = ws;

    ws.onopen = () => {
      setConnected(true);
      console.log("Connected to WebSocket server");
    };

    ws.onmessage = (event) => {
      setMessages((prev) => [...prev, JSON.parse(event.data)]);
    };

    ws.onclose = () => {
      setConnected(false);
      console.log("Disconnected from WebSocket server");
    };

    return () => {
      ws.close();
    };
  }, [isAuth]);

  const addGroupChat = (newGroup: Group) => {
    setGroups((prev) => [...prev, newGroup]);
  };

  const addGroupMember = (memberIds: number[], groupId: number) => {
    const membersToAdd: Friend[] = friends.filter((f) =>
      memberIds.includes(f.friend_id)
    );

    setGroups((prevGroups) =>
      prevGroups.map((group) => {
        if (group.id === groupId) {
          return {
            ...group,
            members: [...group.members, ...membersToAdd],
          };
        }
        return group;
      })
    );
  };

  const editGroupPicture = (groupId: number, pictureUrl: string) => {
    setGroups((prevGroups) =>
      prevGroups.map((group) => {
        if (group.id === groupId) {
          return {
            ...group,
            profile_picture: pictureUrl,
          };
        }
        return group;
      })
    );
  };

  const removeGroupMember = (friendId: number, groupId: number) => {
    setGroups((prev) =>
      prev.map((group) => {
        if (group.id === groupId) {
          return {
            ...group,
            members: group.members.filter(
              (member) => member.friend_id !== friendId
            ),
          };
        }
        return group;
      })
    );
  };

  const logOut = () => {
    localStorage.removeItem("token");
    setShowAuthModal(true);
    setIsAuth(false);
  };

  const sendMessage = (e: React.FormEvent) => {
    e.preventDefault();
    if (inputMessage.trim() && wsRef.current) {
      wsRef.current.send(inputMessage);
      setInputMessage("");
    }
  };

  if (initialLoad) {
    return (
      <div className="flex justify-center items-center h-screen">
        <div className="flex flex-col items-center gap-4">
          <div className="w-12 h-12 border-4 border-primary border-t-transparent rounded-full animate-spin"></div>
          <p className="text-xl font-medium text-primary animate-pulse">
            Loading GChat...
          </p>
        </div>
      </div>
    );
  }

  if (!isAuth) {
    return (
      <div>
        {showAuthModal && (
          <AuthModals
            hideAuthModal={() => {
              setIsAuth(true);
              setShowAuthModal(false);
            }}
          />
        )}
      </div>
    );
  }

  return (
    <div className="h-dvh flex flex-col">
      <SidebarProvider>
        <Sidebar>
          <h1 className="text-md font-semibold text-center pt-4">GChat</h1>
          <div className="flex flex-row justify-between">
            <CreateChatForm addGroupChat={addGroupChat} friends={friends} />
            <ManageFriends
              initialFriends={friends}
              initialIncomingFriends={incomingFriends}
              initialOutgoingFriends={outgoingFriends}
            />
          </div>
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
                        onClick={() => setSelectedChat(chat.id)}
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
          <SidebarFooter>
            <hr />
            <div className="flex flex-row justify-between gap-2">
              <div className="flex flex-row items-center gap-2">
                <Avatar>
                  <AvatarImage src={user?.profile_picture} />
                  <AvatarFallback>
                    {user?.username.substring(0, 2).toUpperCase() ?? ""}
                  </AvatarFallback>
                </Avatar>
                <p
                  className="font-semibold"
                  style={{ color: usernameToColor(user?.username ?? "") }}
                >
                  {user?.username ?? "unknown"}
                </p>
              </div>
              <div className="flex flex-row items-center gap-4">
                <TooltipProvider delayDuration={0}>
                  <SettingsModal />
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <button onClick={logOut}>
                        <LogOut className="h-5 w-5 font-semibold hover:text-primary hover:cursor-pointer" />
                      </button>
                    </TooltipTrigger>
                    <TooltipContent>
                      <p>Log Out</p>
                    </TooltipContent>
                  </Tooltip>
                </TooltipProvider>
              </div>
            </div>
          </SidebarFooter>
        </Sidebar>
        <SidebarInset className="flex flex-col flex-1">
          <header className="flex flex-row h-16 items-center justify-between border-b px-6">
            <div className="flex flex-row gap-2 items-center">
              <SidebarTrigger>
                <Menu className="h-6 w-6" />
              </SidebarTrigger>
            </div>
            <h1 className="font-semibold ">
              {selectedChat ? (
                selectedChat == -1 ? (
                  "Test Chat"
                ) : (
                  <GroupManagementModal
                    group={groups.find((group) => group.id === selectedChat)!}
                    friends={friends}
                    removeGroupMember={removeGroupMember}
                    addGroupMember={addGroupMember}
                    editGroupPicture={editGroupPicture}
                  />
                )
              ) : (
                "Select a chat"
              )}
            </h1>
            <div className="flex flex-row items-center gap-3">
              <Ping connected={connected} />
              <ThemeToggle />
            </div>
          </header>
          <main className="flex-1 overflow-hidden px-6 pb-3 pt-1">
            <div
              ref={scrollRef}
              className="flex h-full w-full overflow-y-auto scrollbar-hidden"
            >
              <div className="flex flex-col gap-2 w-full">
                {!selectedChat ? (
                  <div className="h-full flex flex-col items-center justify-center">
                    <Card className="flex flex-col items-center max-w-md text-center p-8 ">
                      <MessageSquare className="h-16 w-16 text-primary/80 mb-4" />
                      <h2 className="text-2xl font-bold mb-2 text-primary">
                        Welcome to GChat
                      </h2>
                      <p className="mb-6">
                        Select a conversation from the sidebar to start
                        chatting, or create a new conversation.
                      </p>
                    </Card>
                  </div>
                ) : selectedChat !== -1 ? (
                  <div></div>
                ) : (
                  <div className="flex flex-col gap-2 w-full">
                    {messages.map((message) =>
                      message.user_id === getIdFromJWT() ? (
                        <div key={message.id} className="flex justify-end">
                          <div className="flex flex-col">
                            <Card className="bg-primary border-0 shadow-none py-2 px-4 text-white w-fit rounded-tr-none ml-12 text-sm">
                              {message.content}
                            </Card>
                            <p className="text-sm opacity-40 text-right pr-3">
                              {new Date(message.timestamp).toLocaleDateString(
                                [],
                                {
                                  month: "short",
                                  day: "numeric",
                                }
                              ) +
                                "  • " +
                                new Date(message.timestamp).toLocaleTimeString(
                                  [],
                                  {
                                    hour: "2-digit",
                                    minute: "2-digit",
                                  }
                                )}
                            </p>
                          </div>
                        </div>
                      ) : (
                        <div key={message.id} className="flex justify-start">
                          <div className={`flex gap-3 max-w-[80%] flex-row`}>
                            <Avatar className="h-9 w-9 mt-1">
                              <AvatarImage src={message.profile_picture} />
                              <AvatarFallback>
                                {message.username.substring(0, 2).toUpperCase()}
                              </AvatarFallback>
                            </Avatar>
                            <div className="flex flex-col">
                              <p
                                className="text-sm"
                                style={{
                                  color: usernameToColor(message.username),
                                }}
                              >
                                {message.username}
                              </p>
                              <Card className="bg-slate-300 border-0 shadow-none py-2 px-4 text-black w-fit rounded-tl-none text-sm">
                                {message.content}
                              </Card>
                              <p className="text-sm opacity-40">
                                {new Date(message.timestamp).toLocaleDateString(
                                  [],
                                  {
                                    month: "short",
                                    day: "numeric",
                                  }
                                ) +
                                  " • " +
                                  new Date(
                                    message.timestamp
                                  ).toLocaleTimeString([], {
                                    hour: "2-digit",
                                    minute: "2-digit",
                                  })}
                              </p>
                            </div>
                          </div>
                        </div>
                      )
                    )}
                  </div>
                )}
              </div>
            </div>
            <hr />
          </main>
          <footer className="pb-5">
            {selectedChat && (
              <form
                onSubmit={sendMessage}
                className="flex flex-row gap-2 mt-3 w-full justify-center"
              >
                <Input
                  className="w-[60%]"
                  type="text"
                  value={inputMessage}
                  onChange={(e) => setInputMessage(e.target.value)}
                  placeholder="Type your message..."
                  disabled={!connected}
                />
                <Button className="w-[5%]" disabled={!inputMessage.trim()}>
                  <Send />
                </Button>
              </form>
            )}
          </footer>
        </SidebarInset>
      </SidebarProvider>
    </div>
  );
}
