"use client";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { Dice5, Settings } from "lucide-react";
import { User } from "../fetchData";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { z } from "zod";
import { useForm } from "@tanstack/react-form";
import { generateProfilePictureSVG } from "../utils";
import { useState } from "react";
import { Label } from "@/components/ui/label";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import StatsComponent from "./statsComponent";

const userSchema = z.object({
  email: z.string().email(),
  username: z.string(),
  profilePicture: z.string(),
  currentPassword: z.string(),
  newPassword: z.string(),
});

interface Props {
  user: User;
}

export default function SettingsModal({ user }: Props) {
  const [profilePicture, setProfilePicture] = useState(user.profile_picture);
  const [username, setUsername] = useState(user.username);

  const form = useForm({
    defaultValues: {
      email: user.email,
      username: user.username,
      profilePicture: user.profile_picture,
      currentPassword: "",
      newPassword: "",
    },
    validators: {
      onChange: userSchema,
    },
    onSubmit: async (e) => {
      const payload = {
        email: e.value.email,
        username: e.value.username,
        profilePicture: e.value.profilePicture,
        currentPassword: e.value.currentPassword,
        newPassword: e.value.newPassword,
      };
      console.log(payload);
    },

    //ERROR MESSAGES:
    //CURRENT PASSWORD REQUIRED TO CHANGE PASSWORD
    //INCORRECT CURRENT PASSWORD
    //NEW PASSWORD MUST BE AT LEAST 8 CHARACTERS
    //USERNAME ALREADY TAKEN
    //EMAIL ALREADY REGISTERED TO DIFFERENT ACCOUNT
    //AN UNEXPECTED ERROR OCCURED
  });

  return (
    <Dialog>
      <DialogTrigger asChild>
        <button className="hover:cursor-pointer">
          <Tooltip>
            <TooltipTrigger asChild>
              <Settings className="h-5 w-5 font-semibold hover:text-primary" />
            </TooltipTrigger>
            <TooltipContent>
              <p>Settings</p>
            </TooltipContent>
          </Tooltip>
        </button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-md">
        <Tabs defaultValue="settings" className="pr-2">
          <DialogTitle>
            <TabsList className="grid w-full grid-cols-2">
              <TabsTrigger value="settings">Settings</TabsTrigger>
              <TabsTrigger value="stats">Stats</TabsTrigger>
            </TabsList>
          </DialogTitle>
          <TabsContent value="settings">
            <DialogDescription className="text-center pb-4">
              Manage your account information
            </DialogDescription>
            <form
              className="flex flex-col w-full gap-4"
              onSubmit={(e) => {
                e.preventDefault();
                e.stopPropagation();
                form.handleSubmit();
              }}
            >
              <div className="flex justify-center">
                <div className="relative w-20 h-20">
                  <Avatar className="w-full h-full">
                    <AvatarImage src={profilePicture} />
                    <AvatarFallback>
                      {user.username.substring(0, 2).toUpperCase() ?? ""}
                    </AvatarFallback>
                  </Avatar>
                  <form.Field name="profilePicture">
                    {(field) => (
                      <Dice5
                        onClick={() => {
                          const newPfp = generateProfilePictureSVG(username);
                          setProfilePicture(newPfp);
                          field.handleChange(newPfp);
                        }}
                        className="absolute bottom-0 right-0 w-7 h-7 hover:cursor-pointer bg-white rounded-full translate-x-1/3 translate-y-1/3 p-1 text-primary"
                      />
                    )}
                  </form.Field>
                </div>
              </div>

              <form.Field name="email">
                {(field) => (
                  <div className="flex flex-col gap-1">
                    <Label htmlFor={field.name}>Email</Label>
                    <Input
                      id={field.name}
                      onChange={(e) => field.handleChange(e.target.value)}
                      value={field.state.value}
                    />
                  </div>
                )}
              </form.Field>
              <form.Field name="username">
                {(field) => (
                  <div className="flex flex-col gap-1">
                    <Label htmlFor={field.name}>Username</Label>
                    <Input
                      id={field.name}
                      onChange={(e) => {
                        const newUsername = e.target.value;
                        field.handleChange(newUsername);
                        const newPfp = generateProfilePictureSVG(newUsername);
                        setProfilePicture(newPfp);
                        form.setFieldValue("profilePicture", newPfp);
                        setUsername(newUsername);
                      }}
                      value={field.state.value}
                    />
                  </div>
                )}
              </form.Field>
              <form.Field name="currentPassword">
                {(field) => (
                  <div className="flex flex-col gap-1">
                    <Label htmlFor={field.name}>Current Password</Label>
                    <Input
                      id={field.name}
                      onChange={(e) => field.handleChange(e.target.value)}
                      value={field.state.value}
                      placeholder="●●●●●●●●"
                    />
                  </div>
                )}
              </form.Field>
              <form.Field name="newPassword">
                {(field) => (
                  <div className="flex flex-col gap-1">
                    <Label htmlFor={field.name}>New Password</Label>
                    <Input
                      id={field.name}
                      onChange={(e) => field.handleChange(e.target.value)}
                      value={field.state.value}
                      placeholder="●●●●●●●●"
                    />
                  </div>
                )}
              </form.Field>
              <form.Subscribe
                selector={(state) => [state.canSubmit, state.isSubmitting]}
              >
                {([canSubmit, isSubmitting]) => (
                  <Button
                    type="submit"
                    disabled={!canSubmit}
                    className="max-w-32"
                  >
                    {isSubmitting ? "..." : "Save Changes"}
                  </Button>
                )}
              </form.Subscribe>
            </form>
          </TabsContent>
          <TabsContent value="stats">
            <DialogDescription className="text-center pb-4">
              Fun stats about your activity
            </DialogDescription>
            <StatsComponent />
          </TabsContent>
        </Tabs>
      </DialogContent>
    </Dialog>
  );
}
