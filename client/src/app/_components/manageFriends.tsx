"use client";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  SidebarHeader,
  SidebarMenu,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import { Check, MessageSquareShare, Plus, Users, X } from "lucide-react";
import { ScrollArea } from "@/components/ui/scroll-area";
import { FormEvent, useEffect, useState } from "react";
import axios from "axios";
import qs from "qs";

interface Friend {
  friend_id: number;
  username: string;
}

interface FriendRequest {
  receiver_id: number;
  sender_id: number;
  username: string;
}

interface FriendRequestResponse {
  incoming: FriendRequest[];
  outgoing: FriendRequest[];
}

interface AddFriendResponse {
  message: string;
  friend_request: FriendRequest;
}

interface Props {
  initialFriends: Friend[];
}

export default function ManageFriends({ initialFriends }: Props) {
  const [incomingFriends, setIncomingFriends] = useState<FriendRequest[]>([]);
  const [outgoingFriends, setOutgoingFriends] = useState<FriendRequest[]>([]);
  const [friends, setFriends] = useState<Friend[]>(initialFriends);
  const [formMessage, setFormMessage] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    const token = localStorage.getItem("token");
    if (!token) return;

    axios
      .get<FriendRequestResponse>(
        `https://api.gchat.cloud/friend/get-requests?token=${token}`
      )
      .then((response) => {
        setIncomingFriends(response.data.incoming);
        setOutgoingFriends(response.data.outgoing);
      })
      .catch((error) => {
        console.error("An unexpected error occurred:", error);
      });
  }, []);

  function addFriend(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    setIsLoading(true);
    setFormMessage("");

    const token = localStorage.getItem("token");
    if (!token) {
      setIsLoading(false);
      return;
    }
    const username = e.currentTarget.username.value;
    if (username === "") {
      setIsLoading(false);
      return;
    }

    const payload = qs.stringify({
      receiverUsername: username,
      token: token,
    });

    axios
      .post<AddFriendResponse>(
        "https://api.gchat.cloud/friend/send-request",
        payload
      )
      .then((response) => {
        setFormMessage(response.data.message);
        setOutgoingFriends([...outgoingFriends, response.data.friend_request]);
      })
      .catch((error) => {
        setFormMessage(error.response.data.message);
      })
      .finally(() => setIsLoading(false));
  }

  const handleAccept = async (sender_id: number, username: string) => {
    const payload = qs.stringify({
      userId: sender_id,
      token: localStorage.getItem("token"),
    });
    axios
      .post("https://api.gchat.cloud/friend/accept-request", payload)
      .then((response) => {
        if (response.status === 200) {
          setIncomingFriends(
            incomingFriends.filter((friend) => friend.sender_id !== sender_id)
          );
          setFriends([
            ...friends,
            { friend_id: sender_id, username: username },
          ]);
        }
      })
      .catch((error) => {
        console.error("An unexpected error occurred:", error);
      });
  };

  const handleCancel = async (receiver_id: number) => {
    const payload = qs.stringify({
      userId: receiver_id,
      token: localStorage.getItem("token"),
    });
    axios
      .post("https://api.gchat.cloud/friend/cancel-request", payload)
      .then((response) => {
        if (response.status === 200) {
          setOutgoingFriends(
            outgoingFriends.filter(
              (friend) => friend.receiver_id !== receiver_id
            )
          );
        }
      })
      .catch((error) => {
        console.error("An unexpected error occurred:", error);
      });
  };

  const handleDeny = async (sender_id: number) => {
    const payload = qs.stringify({
      userId: sender_id,
      token: localStorage.getItem("token"),
    });
    axios
      .post("https://api.gchat.cloud/friend/deny-request", payload)
      .then((response) => {
        if (response.status === 200) {
          setIncomingFriends(
            incomingFriends.filter((friend) => friend.sender_id !== sender_id)
          );
        }
      })
      .catch((error) => {
        console.error("An unexpected error occurred:", error);
      });
  };

  const handleRemove = async (friendId: number) => {
    const payload = qs.stringify({
      userId: friendId,
      token: localStorage.getItem("token"),
    });
    axios
      .post("https://api.gchat.cloud/friend/delete", payload)
      .then((response) => {
        if (response.status === 200) {
          setFriends(friends.filter((friend) => friend.friend_id !== friendId));
        }
      })
      .catch((error) => {
        console.error("An unexpected error occurred:", error);
      });
  };

  return (
    <Dialog>
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <DialogTrigger asChild>
              <div className="w-full items-center">
                <Button
                  variant="outline"
                  className="flex-1 gap-2 hover:bg-primary/10"
                >
                  <Users className="h-4 w-4 text-primary" />
                  <span>Friends</span>
                </Button>
              </div>
            </DialogTrigger>
            <DialogContent>
              <DialogTitle className="text-center">Manage Friends</DialogTitle>
              <Tabs defaultValue="Group Chat" className="pr-2">
                <DialogHeader>
                  <TabsList className="grid w-full grid-cols-2">
                    <TabsTrigger value="friendList">Friend List</TabsTrigger>
                    <TabsTrigger value="addFriends">Add Friends</TabsTrigger>
                  </TabsList>
                </DialogHeader>
                <TabsContent value="friendList">
                  <Card>
                    <CardHeader>
                      <CardTitle>Manage your friends</CardTitle>
                      <CardDescription>
                        Start a chat or remove connections
                      </CardDescription>
                    </CardHeader>
                    <CardContent>
                      <ScrollArea className="h-[270px] pr-4">
                        <div className="space-y-2">
                          {friends.map((friend) => (
                            <div
                              key={friend.friend_id}
                              className="space-y-1 flex flex-row gap-3 pb-2 items-center border-b justify-between"
                            >
                              <Label>{friend.username}</Label>
                              <div className="flex flex-row gap-3">
                                <TooltipProvider delayDuration={0}>
                                  <Tooltip>
                                    <TooltipTrigger asChild>
                                      <MessageSquareShare className="h-4 w-4 font-semibold hover:text-primary hover:cursor-pointer" />
                                    </TooltipTrigger>
                                    <TooltipContent>
                                      <p>Go to chat</p>
                                    </TooltipContent>
                                  </Tooltip>
                                  <Tooltip>
                                    <TooltipTrigger asChild>
                                      <button
                                        onClick={() =>
                                          handleRemove(friend.friend_id)
                                        }
                                      >
                                        <X className="h-4 w-4 font-semibold hover:text-primary hover:cursor-pointer" />
                                      </button>
                                    </TooltipTrigger>
                                    <TooltipContent>
                                      <p>Remove Friend</p>
                                    </TooltipContent>
                                  </Tooltip>
                                </TooltipProvider>
                              </div>
                            </div>
                          ))}
                        </div>
                      </ScrollArea>
                    </CardContent>
                  </Card>
                </TabsContent>
                <TabsContent value="addFriends">
                  <Card>
                    <CardHeader>
                      <CardTitle>Add Friends</CardTitle>
                      <CardDescription>
                        Add new friends to your friend list and accept friend
                        requests
                      </CardDescription>
                    </CardHeader>
                    <CardContent className="space-y-5">
                      <div className="flex flex-col gap-1">
                        <Label className="pl-1">Add friend by username</Label>
                        <form
                          className="flex flex-row gap-3"
                          onSubmit={(e) => addFriend(e)}
                        >
                          <Input
                            type="text"
                            name="username"
                            placeholder="Friend's username"
                            className="w-full"
                          />
                          <Button type="submit" disabled={isLoading}>
                            {isLoading ? "..." : <Plus />}
                          </Button>
                        </form>
                        <p className="text-sm">{formMessage}</p>
                      </div>
                      <hr></hr>
                      <Tabs defaultValue="incoming" className="pr-2">
                        <TabsList className="grid w-full grid-cols-2">
                          <TabsTrigger value="incoming">
                            Incoming Requests
                          </TabsTrigger>
                          <TabsTrigger value="outgoing">
                            Outgoing Requests
                          </TabsTrigger>
                        </TabsList>
                        <TabsContent value="incoming">
                          <div>
                            <ScrollArea className="h-[215px] pr-4 pt-5">
                              <div className="space-y-2">
                                {incomingFriends.map((friend) => (
                                  <div
                                    key={friend.sender_id}
                                    className="space-y-1 flex flex-row gap-3 pb-2 items-center border-b justify-between"
                                  >
                                    <Label>{friend.username}</Label>
                                    <div className="flex flex-row gap-3">
                                      <TooltipProvider delayDuration={0}>
                                        <Tooltip>
                                          <TooltipTrigger asChild>
                                            <button
                                              onClick={() =>
                                                handleAccept(
                                                  friend.sender_id,
                                                  friend.username
                                                )
                                              }
                                            >
                                              <Check className="h-4 w-4 font-semibold hover:text-primary hover:cursor-pointer" />
                                            </button>
                                          </TooltipTrigger>
                                          <TooltipContent>
                                            <p>Accept Request</p>
                                          </TooltipContent>
                                        </Tooltip>
                                        <Tooltip>
                                          <TooltipTrigger asChild>
                                            <button
                                              onClick={() =>
                                                handleDeny(friend.sender_id)
                                              }
                                            >
                                              <X className="h-4 w-4 font-semibold hover:text-primary hover:cursor-pointer" />
                                            </button>
                                          </TooltipTrigger>
                                          <TooltipContent>
                                            <p>Deny Request</p>
                                          </TooltipContent>
                                        </Tooltip>
                                      </TooltipProvider>
                                    </div>
                                  </div>
                                ))}
                              </div>
                            </ScrollArea>
                          </div>
                        </TabsContent>
                        <TabsContent value="outgoing">
                          <div>
                            <ScrollArea className="h-[215px] pr-4 pt-5">
                              <div className="space-y-2">
                                {outgoingFriends.map((friend) => (
                                  <div
                                    key={friend.receiver_id}
                                    className="space-y-1 flex flex-row gap-3 pb-2 items-center border-b justify-between"
                                  >
                                    <Label>{friend.username}</Label>
                                    <div className="flex flex-row gap-3">
                                      <TooltipProvider delayDuration={0}>
                                        <Tooltip>
                                          <TooltipTrigger asChild>
                                            <button
                                              onClick={() =>
                                                handleCancel(friend.receiver_id)
                                              }
                                            >
                                              <X className="h-4 w-4 font-semibold hover:text-primary hover:cursor-pointer" />
                                            </button>
                                          </TooltipTrigger>
                                          <TooltipContent>
                                            <p>Cancel Request</p>
                                          </TooltipContent>
                                        </Tooltip>
                                      </TooltipProvider>
                                    </div>
                                  </div>
                                ))}
                              </div>
                            </ScrollArea>
                          </div>
                        </TabsContent>
                      </Tabs>
                    </CardContent>
                  </Card>
                </TabsContent>
              </Tabs>
            </DialogContent>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
    </Dialog>
  );
}
