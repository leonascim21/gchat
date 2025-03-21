"use client";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { z } from "zod";
import { useForm } from "@tanstack/react-form";
import axios from "axios";
import qs from "qs";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { Settings } from "lucide-react";

const userSchema = z
  .object({
    email: z.string().email().min(1, "Email is required"),
    username: z.string().min(1, "Username is required"),
    password: z.string().min(8, "Password must be at least 8 characters"),
    confirmPassword: z.string().min(1, "Passwords do not match"),
  })
  .superRefine(({ confirmPassword, password }, ctx) => {
    if (confirmPassword !== password) {
      ctx.addIssue({
        code: "custom",
        message: "Passwords do not match",
        path: ["confirmPassword"],
      });
    }
  });

export default function SettingsModal() {
  const form = useForm({
    defaultValues: {
      email: "",
      username: "",
      password: "",
      confirmPassword: "",
    },
    validators: {
      onChange: userSchema,
    },
    onSubmit: async (e) => {
      try {
        const data = qs.stringify(e.value);
        const response = await axios.post(
          "http://206.189.202.251:3000/register",
          data
        );
        console.log("Registration successful:", response.data);
      } catch (error) {
        if (axios.isAxiosError(error)) {
          console.error("Registration failed:", error);
          if (error.response) {
            console.error("Server response:", error.response.data);
            console.error("Server status:", error.response.status);
            console.error("Server headers:", error.response.headers);
          } else if (error.request) {
            console.error("No response received:", error.request);
          } else {
            console.error("Error setting up the request:", error.message);
          }
        } else {
          console.error("An unexpected error occurred:", error);
        }
      }
    },
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
        <DialogTitle className="text-2xl text-center">Settings</DialogTitle>
        <DialogDescription className="text-center">
          Manage your account information
        </DialogDescription>
        <div className="flex items-center justify-center">Coming Soon</div>
      </DialogContent>
    </Dialog>
  );
}
