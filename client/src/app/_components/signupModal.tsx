"use client";
import React, { useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { z } from "zod";
import { useForm } from "@tanstack/react-form";
import axios from "axios";
import qs from "qs";
import { generateProfilePictureSVG } from "../utils";

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

interface Props {
  showSignIn: () => void;
  successfullSignUp: () => void;
}

export default function SignUpModal({ showSignIn, successfullSignUp }: Props) {
  const [open, setOpen] = useState(true);

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
        const data = {
          email: e.value.email,
          username: e.value.username,
          password: e.value.password,
          confirmPassword: e.value.confirmPassword,
          profilePicture: generateProfilePictureSVG(e.value.username),
        };
        const payload = qs.stringify(data);
        const response = await axios.post(
          "https://api.gchat.cloud/user/register",
          payload
        );
        console.log("Registration successful");
        localStorage.setItem("token", response.data.token);
        successfullSignUp();
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
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        onPointerDownOutside={(e) => {
          e.preventDefault();
        }}
        onEscapeKeyDown={(e) => {
          e.preventDefault();
        }}
        className="sm:max-w-md [&>button]:hidden"
      >
        <DialogTitle className="text-2xl text-center">GChat</DialogTitle>
        <DialogDescription className="text-center">
          Create a new account
        </DialogDescription>
        <form
          onSubmit={(e) => {
            e.preventDefault();
            e.stopPropagation();
            form.handleSubmit();
          }}
        >
          <div className="flex flex-col gap-4">
            <form.Field name="email">
              {(field) => {
                return (
                  <div className="grid gap-2">
                    <Label htmlFor={field.name}>Email</Label>
                    <div>
                      <Input
                        id={field.name}
                        onChange={(e) => field.handleChange(e.target.value)}
                        value={field.state.value}
                      />
                      {field.state.meta.isDirty && field.state.meta.errors && (
                        <p className="text-red-500 text-sm">
                          {field.state.meta.errors[0]?.message}
                        </p>
                      )}
                    </div>
                  </div>
                );
              }}
            </form.Field>
            <form.Field name="username">
              {(field) => {
                return (
                  <div className="grid gap-2">
                    <Label htmlFor={field.name}>Username</Label>
                    <div>
                      <Input
                        id={field.name}
                        onChange={(e) => field.handleChange(e.target.value)}
                        value={field.state.value}
                      />
                      {field.state.meta.isDirty && field.state.meta.errors && (
                        <p className="text-red-500 text-sm">
                          {field.state.meta.errors[0]?.message}
                        </p>
                      )}
                    </div>
                  </div>
                );
              }}
            </form.Field>
            <form.Field name="password">
              {(field) => {
                return (
                  <div className="grid gap-2">
                    <Label htmlFor={field.name}>Password</Label>
                    <div>
                      <Input
                        id={field.name}
                        onChange={(e) => field.handleChange(e.target.value)}
                        value={field.state.value}
                        type="password"
                      />
                      {field.state.meta.isDirty && field.state.meta.errors && (
                        <p className="text-red-500 text-sm">
                          {field.state.meta.errors[0]?.message}
                        </p>
                      )}
                    </div>
                  </div>
                );
              }}
            </form.Field>
            <form.Field name="confirmPassword">
              {(field) => {
                return (
                  <div className="grid gap-2">
                    <Label htmlFor={field.name}>Confirm Password</Label>
                    <div>
                      <Input
                        id={field.name}
                        onChange={(e) => field.handleChange(e.target.value)}
                        value={field.state.value}
                        type="password"
                      />
                      {field.state.meta.isDirty && field.state.meta.errors && (
                        <p className="text-red-500 text-sm">
                          {field.state.meta.errors[0]?.message}
                        </p>
                      )}
                    </div>
                  </div>
                );
              }}
            </form.Field>
            <form.Subscribe
              selector={(state) => [state.canSubmit, state.isSubmitting]}
            >
              {([canSubmit, isSubmitting]) => (
                <Button type="submit" disabled={!canSubmit}>
                  {isSubmitting ? "..." : "Sign Up"}
                </Button>
              )}
            </form.Subscribe>
          </div>
        </form>
        <div className="mt-4 text-center text-sm">
          Already have an account?{" "}
          <button onClick={showSignIn} className="underline underline-offset-4">
            Sign In
          </button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
