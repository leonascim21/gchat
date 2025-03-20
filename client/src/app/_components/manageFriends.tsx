import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
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
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import { Check, CircleUser, MessageSquareShare, Plus, X } from "lucide-react";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
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
              <SidebarMenuButton className="w-full items-center">
                <CircleUser className="h-4 w-4 text-primary font-bold" />
                <div className="text-primary font-bold">Friends</div>
              </SidebarMenuButton>
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
                      <ScrollArea className="h-[250px] pr-4">
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
                      <div>
                        <Label className="font-semibold text-primary">
                          Incoming Friend Requests
                        </Label>
                        <ScrollArea className="h-[120px] pr-4 pt-5">
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
                      <hr></hr>
                      <div>
                        <Label className="font-semibold text-primary">
                          Outgoing Friend Requests
                        </Label>
                        <ScrollArea className="h-[120px] pr-4 pt-5">
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
