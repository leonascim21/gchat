"use client";

import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { ScrollArea } from "@/components/ui/scroll-area";
import { useState } from "react";

interface Friend {
  friend_id: number;
  username: string;
  profile_picture?: string;
}

interface Props {
  friends: Friend[];
  groupId: number;
}

export function GroupSettingsForm({ friends, groupId }: Props) {
  const [usersOpen, setUsersOpen] = useState(false);
  const [pictureOpen, setPictureOpen] = useState(false);

  return (
    <div>
      <div className="flex flex-row justify-between">
        <Button
          onClick={() => setUsersOpen(!usersOpen)}
          className="w-[30%] mb-4"
          variant="outline"
        >
          Add Users
        </Button>
        <Button
          onClick={() => setPictureOpen(!pictureOpen)}
          className="w-[30%] mb-4"
          variant="outline"
        >
          Change Picture
        </Button>
      </div>
      {pictureOpen && (
        <form
          onSubmit={
            (e) => console.log(e)
            //createGroupChat(e)
          }
          className="flex flex-row gap-4"
        >
          <Input
            type="text"
            name="pictureURL"
            placeholder="Picture URL"
            className="w-[70%]"
          />
          <Button className="w-[30%]" type="submit">
            Change Picture
          </Button>
        </form>
      )}
      {usersOpen && (
        <div>
          <form
            onSubmit={
              (e) => console.log(e)
              //createGroupChat(e)
            }
            className="flex flex-col gap-4"
          >
            <h1>{groupId}</h1>
            <ScrollArea className="h-[150px] pr-4">
              <div className="space-y-2">
                {friends.map((friend) => (
                  <div
                    key={friend.friend_id}
                    className="space-y-1 flex flex-row gap-3 pb-2 items-center border-b"
                  >
                    <Checkbox
                      onCheckedChange={(e) =>
                        //handleCheckboxChange(friend.friend_id, e as boolean)
                        console.log(e)
                      }
                    />

                    <Label>{friend.username}</Label>
                  </div>
                ))}
              </div>
            </ScrollArea>
            <Button className="w-[30%]" type="submit">
              Add Friends
            </Button>
          </form>
        </div>
      )}
    </div>
  );
}
