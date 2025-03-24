"use client";

import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { LogOut, Menu, MessageSquare, Send } from "lucide-react";
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
import { groupMessages, groups, messages, users } from "./fakeData/fakeData";
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

const chats = groups;
const message = messages;
const groupMessage = groupMessages;
const usera = users;

interface Message {
  id: number;
  user_id: number;
  username: string;
  content: string;
  timestamp: string;
  profile_picture: string;
}

interface User {
  userID: number;
  username: string;
  profile_picture: string;
  email: string;
}

export default function Home() {
  const [initialLoad, setInitialLoad] = useState(true);
  const [selectedChat, setSelectedChat] = useState<number | null>(null);
  const [searchQuery, setSearchQuery] = useState("");
  useState<number | null>(null);
  const [messages, setMessages] = useState<Message[]>([]);
  const [user, setUser] = useState<User | null>(null);
  const [inputMessage, setInputMessage] = useState("");
  const [connected, setConnected] = useState(false);
  const [showAuthModal, setShowAuthModal] = useState(false);
  const [isAuth, setIsAuth] = useState(false);

  const wsRef = useRef<WebSocket | null>(null);

  const scrollRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [selectedChat, messages]);

  useEffect(() => {
    const token = localStorage.getItem("token");
    if (!token) {
      console.log("Token not found");
      setShowAuthModal(true);
      setInitialLoad(false);
      return;
    }
    axios
      .get(`https://api.gchat.cloud/check-token?token=${token}`)
      .then((response) => {
        if (response.data.valid) {
          setIsAuth(true);
          console.log("Token is valid");
        } else {
          setShowAuthModal(true);
          console.log("Token is invalid");
        }
      })
      .catch((error) => {
        setShowAuthModal(true);
        console.error("Error checking token:", error);
      })
      .finally(() => setInitialLoad(false));
  }, []);

  useEffect(() => {
    const token = localStorage.getItem("token");
    if (!token || !isAuth) return;

    axios
      .get(`https://api.gchat.cloud/get-all-messages?token=${token}`)
      .then((response) => {
        setMessages(response.data);
      })
      .catch((error) => {
        console.error("Error fetching messages:", error);
      });

    axios
      .get(`http://api.gchat.cloud/get-user-info?token=${token}`)
      .then((response) => {
        setUser(response.data);
      })
      .catch((error) => {
        console.error("Error fetching user:", error);
      });
  }, [isAuth]);

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

    /*
    const payload = {
      message: inputMessage,
      group_id: selectedChat,
      sender_id: 1,
    };
    if (inputMessage.trim() && wsRef.current) {
      console.log("sent");
      wsRef.current.send(JSON.stringify(payload));
      setInputMessage("");
    }
      */
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
            <CreateChatForm />
            <ManageFriends />
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
              {chats
                .filter((chat) =>
                  chat.group_name
                    .toLowerCase()
                    .includes(searchQuery.toLowerCase())
                )
                .map((chat) => (
                  <div key={chat.groupID}>
                    <SidebarMenuItem>
                      <SidebarMenuButton
                        onClick={() => setSelectedChat(chat.groupID)}
                        isActive={selectedChat === chat.groupID}
                      >
                        <MessageSquare className="mr-2 h-4 w-4" />
                        {chat.group_name}
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
              {selectedChat
                ? chats.find((chat) => chat.groupID === selectedChat)
                    ?.group_name
                : "Select a chat"}
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
                  chats.find((chat) => chat.groupID === selectedChat)
                    ?.groupID &&
                  message
                    .filter((msg) =>
                      groupMessage.find(
                        (gm) =>
                          gm.messageID === msg.messageID &&
                          gm.groupID === selectedChat
                      )
                    )
                    .map((message) =>
                      message.senderID == 1 ? (
                        <div
                          key={message.messageID}
                          className="flex justify-end"
                        >
                          <div className="flex flex-col">
                            <Card className="bg-primary border-0 shadow-lg py-2 px-4 text-white w-fit rounded-full ml-12">
                              {message.content}
                            </Card>
                            <p className="text-sm opacity-40 text-right pr-3">
                              {new Date(message.sent_at).toLocaleDateString(
                                [],
                                {
                                  month: "short",
                                  day: "numeric",
                                }
                              ) +
                                " • " +
                                new Date(message.sent_at).toLocaleTimeString(
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
                        <div
                          key={message.messageID}
                          className="flex flex-col justify-start"
                        >
                          <p className="text-sm pl-3">
                            {
                              usera.find(
                                (person) => person.userID === message.senderID
                              )?.username
                            }
                          </p>
                          <Card className="bg-slate-300 border-0 shadow-lg py-2 px-4 text-black w-fit rounded-full mr-12">
                            {message.content}
                          </Card>
                          <p className="text-sm opacity-40 pl-3">
                            {new Date(message.sent_at).toLocaleDateString([], {
                              month: "short",
                              day: "numeric",
                            }) +
                              " • " +
                              new Date(message.sent_at).toLocaleTimeString([], {
                                hour: "2-digit",
                                minute: "2-digit",
                              })}
                          </p>
                        </div>
                      )
                    )
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
