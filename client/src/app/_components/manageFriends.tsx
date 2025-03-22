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
import { users } from "../fakeData/fakeData";

const friends = users;

export default function ManageFriends() {
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
                              key={friend.userID}
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
                                      <X className="h-4 w-4 font-semibold hover:text-primary hover:cursor-pointer" />
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
                      <div>
                        <Label className="pl-1">Add friend by username</Label>
                        <div className="flex flex-row gap-3">
                          <Input
                            placeholder="Friend's username"
                            className="w-full"
                          />
                          <Button>
                            <Plus />
                          </Button>
                        </div>
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
                                {friends.map((friend) => (
                                  <div
                                    key={friend.userID}
                                    className="space-y-1 flex flex-row gap-3 pb-2 items-center border-b justify-between"
                                  >
                                    <Label>{friend.username}</Label>
                                    <div className="flex flex-row gap-3">
                                      <TooltipProvider delayDuration={0}>
                                        <Tooltip>
                                          <TooltipTrigger asChild>
                                            <Check className="h-4 w-4 font-semibold hover:text-primary hover:cursor-pointer" />
                                          </TooltipTrigger>
                                          <TooltipContent>
                                            <p>Accept Request</p>
                                          </TooltipContent>
                                        </Tooltip>
                                        <Tooltip>
                                          <TooltipTrigger asChild>
                                            <X className="h-4 w-4 font-semibold hover:text-primary hover:cursor-pointer" />
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
                                {friends.map((friend) => (
                                  <div
                                    key={friend.userID}
                                    className="space-y-1 flex flex-row gap-3 pb-2 items-center border-b justify-between"
                                  >
                                    <Label>{friend.username}</Label>
                                    <div className="flex flex-row gap-3">
                                      <TooltipProvider delayDuration={0}>
                                        <Tooltip>
                                          <TooltipTrigger asChild>
                                            <X className="h-4 w-4 font-semibold hover:text-primary hover:cursor-pointer" />
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
