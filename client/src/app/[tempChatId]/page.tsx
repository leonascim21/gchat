"use client";
import { useEffect, useState } from "react";
import Chat from "../_components/chat";
import { useRouter, useParams } from "next/navigation";
import axios from "axios";
import { Group, User } from "../fetchData";
import PasswordModal from "./passwordModal";
import { set } from "zod";

export default function TemporaryChat() {
  const [isLoading, setIsLoading] = useState(true);
  const [user, setUser] = useState<User | null>(null);
  const [group, setGroup] = useState<Group | null>(null);
  const [isPasswordProtected, setIsPasswordProtected] = useState(true);
  const [endTimestamp, setEndTimestamp] = useState<number | null>(null);
  const router = useRouter();
  const params = useParams();
  const { tempChatId } = params;

  useEffect(() => {
    const token = localStorage.getItem("token");
    if (!token) {
      router.push("/");
    }
    axios
      .get(`https://api.gchat.cloud/user/get-user-info?token=${token}`)
      .then((response) => {
        setUser(response.data);
      })
      .catch(() => {
        router.push("/");
      });

    axios
      .get(
        `https://api.gchat.cloud/group/temp-chat-password?token=${token}&temp=${tempChatId}`
        //RETURN BOOL
      )
      .then((response) => {
        if (!response.data.password_protected) {
          setIsPasswordProtected(false);
          axios
            .get(
              `https://api.gchat.cloud/group/get-temp-chat?token=${token}&temp=${tempChatId}`
              //RETURN GROUP + END_DATE TIMESTAMP
            )
            .then((response) => {
              setGroup(response.data.group);
              setEndTimestamp(response.data.end_date);
              setIsLoading(false);
            })
            .catch(() => {
              router.push("/");
            });
        } else {
          setIsPasswordProtected(true);
          setIsLoading(false);
        }
      })
      .catch(() => {
        router.push("/");
      });
  }, []);

  const handlePasswordSubmit = async (password: string) => {
    const token = localStorage.getItem("token");
    if (!token) {
      router.push("/");
    }

    axios
      .get(
        `https://api.gchat.cloud/group/get-temp-chat?token=${token}&temp=${tempChatId}&password=${password}`
      )
      .then((response) => {
        setGroup(response.data.group);
        setEndTimestamp(response.data.end_date);
        setIsPasswordProtected(false);
      })
      .catch((error) => {
        if (error.status === 401) {
          return "Invalid password";
        } else {
          router.push("/");
          return "An unexpected error occured";
        }
      });
    return "";
  };

  if (isLoading) {
    return (
      <div className="flex justify-center items-center h-screen">
        <div className="flex flex-col items-center gap-4">
          <div className="w-12 h-12 border-4 border-primary border-t-transparent rounded-full animate-spin"></div>
          <p className="text-xl font-medium text-primary animate-pulse">
            Loading GChat...
          </p>
        </div>
      </div>
    );
  }

  if (isPasswordProtected) {
    return <PasswordModal handleSubmit={handlePasswordSubmit} />;
  }

  return (
    <div className="flex flex-col h-screen justify-center">
      <div className="flex flex-col p-3  items-center justify-center">
        <h1>{group?.name ?? "undefined"}</h1>
        <h2 className="text-sm">{endTimestamp}</h2>
      </div>
      <Chat
        groupId={group?.id ?? -1}
        user={user}
        updatePing={() => console.log("h1")}
      />
    </div>
  );
}
