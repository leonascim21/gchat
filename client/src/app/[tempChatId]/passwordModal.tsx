"use client";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { useState } from "react";

interface Props {
  handleSubmit: (password: string) => Promise<string>;
}

export default function PasswordModal({ handleSubmit }: Props) {
  const [open, setOpen] = useState(true);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const submitForm = async (password: string) => {
    setIsLoading(true);
    const response = await handleSubmit(password);
    setIsLoading(false);
    setError(response);
  };

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
          This chat is password protected. Please enter the password to
          continue.
        </DialogDescription>
        <form
          onSubmit={(e) => {
            e.preventDefault();
            submitForm(e.currentTarget.password.value);
          }}
          className="flex flex-col gap-4"
        >
          <Input
            type="password"
            name="password"
            placeholder="Password"
            className="w-full"
            required
          />
          <Button className="w-fit mx-auto" type="submit" disabled={isLoading}>
            {isLoading ? "..." : "Submit"}
          </Button>
          {error && (
            <p className="text-red-500 text-sm justify-center flex">{error}</p>
          )}
        </form>
      </DialogContent>
    </Dialog>
  );
}
