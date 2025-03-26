"use client";

import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { ScrollArea } from "@/components/ui/scroll-area";
import { FormEvent, useState } from "react";
import type { Friend } from "../fetchData";
import axios from "axios";
import qs from "qs";

interface Props {
  friends: Friend[];
  groupId: number;
  addGroupMember: (memberIds: number[], groupId: number) => void;
  editGroupPicture: (groupId: number, pictureUrl: string) => void;
}

export function GroupSettingsForm({
  friends,
  groupId,
  addGroupMember,
  editGroupPicture,
}: Props) {
  const [usersOpen, setUsersOpen] = useState(false);
  const [pictureOpen, setPictureOpen] = useState(false);
  const [selectedMemberIds, setSelectedMemberIds] = useState<number[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  const handleCheckboxChange = (friendId: number, isChecked: boolean) => {
    if (isChecked) {
      setSelectedMemberIds([...selectedMemberIds, friendId]);
    } else {
      setSelectedMemberIds(selectedMemberIds.filter((id) => id !== friendId));
    }
  };

  function addMember(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();

    const token = localStorage.getItem("token");
    if (!token) {
      return;
    }
    setIsLoading(true);

    const memberIds = selectedMemberIds;
    const payload = {
      token: token,
      newMemberIds: memberIds,
      groupId: groupId,
    };
    console.log(payload);
    axios
      .post("https://api.gchat.cloud/group/add-users", payload)
      .then((response) => {
        if (response.status === 200) {
          addGroupMember(memberIds, groupId);
        }
      })
      .catch((error) => {
        console.error(error);
      })
      .finally(() => setIsLoading(false));
  }

  function editPicture(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();

    const token = localStorage.getItem("token");
    if (!token) {
      return;
    }
    setIsLoading(true);

    const pictureUrl = e.currentTarget.pictureUrl.value;
    const payload = {
      token: token,
      groupId: groupId,
      pictureUrl: pictureUrl,
    };
    console.log(payload);
    axios
      .put("https://api.gchat.cloud/group/edit-picture", qs.stringify(payload))
      .then((response) => {
        if (response.status === 200) {
          editGroupPicture(groupId, pictureUrl);
        }
      })
      .catch((error) => {
        console.error(error);
      })
      .finally(() => setIsLoading(false));
  }

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
        <form onSubmit={(e) => editPicture(e)} className="flex flex-row gap-4">
          <Input
            type="text"
            name="pictureUrl"
            placeholder="Picture URL"
            className="w-[70%]"
          />
          <Button className="w-[30%]" type="submit" disabled={isLoading}>
            {isLoading ? "..." : "Change Picture"}
          </Button>
        </form>
      )}
      {usersOpen && (
        <div>
          <form onSubmit={(e) => addMember(e)} className="flex flex-col gap-4">
            <ScrollArea className="h-[150px] pr-4">
              <div className="space-y-2">
                {friends.map((friend) => (
                  <div
                    key={friend.friend_id}
                    className="space-y-1 flex flex-row gap-3 pb-2 items-center border-b"
                  >
                    <Checkbox
                      onCheckedChange={(e) =>
                        handleCheckboxChange(friend.friend_id, e as boolean)
                      }
                    />

                    <Label>{friend.username}</Label>
                  </div>
                ))}
              </div>
            </ScrollArea>
            <Button className="w-[30%]" type="submit" disabled={isLoading}>
              {isLoading ? "..." : "Add Friends"}
            </Button>
          </form>
        </div>
      )}
    </div>
  );
}
