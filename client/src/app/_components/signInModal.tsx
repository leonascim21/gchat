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

const userSchema = z.object({
  username: z.string().min(1, "Username is required"),
  password: z.string().min(1, "Password is required"),
});

interface Props {
  showSignUp: () => void;
  successfullSignIn: () => void;
}

export default function SignInModal({ showSignUp, successfullSignIn }: Props) {
  const [open, setOpen] = useState(true);

  const form = useForm({
    defaultValues: {
      username: "",
      password: "",
    },
    validators: {
      onChange: userSchema,
    },
    onSubmit: async (e) => {
      try {
        const data = qs.stringify(e.value);
        const response = await axios.post(
          "http://206.189.202.251:3000/login",
          data
        );
        console.log("Login successful");
        localStorage.setItem("token", response.data.token);
        successfullSignIn();
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
          Enter your account details below to sign in
        </DialogDescription>
        <form
          onSubmit={(e) => {
            e.preventDefault();
            e.stopPropagation();
            form.handleSubmit();
          }}
        >
          <div className="flex flex-col gap-4">
            <form.Field name="username">
              {(field) => (
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
              )}
            </form.Field>
            <form.Field name="password">
              {(field) => (
                <div className="flex flex-col gap-6">
                  <div className="grid gap-2">
                    <div className="flex justify-between items-center">
                      <Label htmlFor={field.name}>Password</Label>
                      <a
                        href="#"
                        className="text-sm underline-offset-4 hover:underline"
                      >
                        Forgot your password?
                      </a>
                    </div>
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
                </div>
              )}
            </form.Field>
            <form.Subscribe
              selector={(state) => [state.canSubmit, state.isSubmitting]}
            >
              {([canSubmit, isSubmitting]) => (
                <Button type="submit" disabled={!canSubmit}>
                  {isSubmitting ? "..." : "Sign In"}
                </Button>
              )}
            </form.Subscribe>
          </div>
        </form>
        <div className="mt-4 text-center text-sm">
          Don&apos;t have an account?{" "}
          <button onClick={showSignUp} className="underline underline-offset-4">
            Sign up
          </button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
