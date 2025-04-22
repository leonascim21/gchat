"use client";
import { useEffect, useState } from "react";
import Chat from "../_components/chat";
import { useRouter, useParams } from "next/navigation";
import axios from "axios";
import { User } from "../fetchData";
import PasswordModal from "./passwordModal";
import Ping from "../_components/ping";

interface Response {
  chat_key: string;
  group_id: number;
  end_date: string;
  name: string;
}

export default function TemporaryChat() {
  const [isLoading, setIsLoading] = useState(true);
  const [user, setUser] = useState<User | null>(null);
  const [groupInfo, setGroupInfo] = useState<Response | null>(null);
  const [isPasswordProtected, setIsPasswordProtected] = useState(true);
  const [password, setPassword] = useState<string | undefined>(undefined);
  const [connected, setConnected] = useState(false);

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
      })
      .finally(() => setIsLoading(false));
  }, []);

  const handlePasswordSubmit = async (password: string) => {
    const token = localStorage.getItem("token");
    if (!token) {
      router.push("/");
    }

    const response = await axios
      .get(
        `https://api.gchat.cloud/temp-group/get-group-info?password=${password}&temp=${tempChatId}`
      )
      .then((response) => {
        setGroupInfo(response.data);
        setPassword(password);
        setIsPasswordProtected(false);
      })
      .catch((error) => {
        if (error.response.status === 401) {
          return "Invalid password";
        } else {
          router.push("/");
          return "An unexpected error occured";
        }
      });
    if (response) {
      return response;
    }
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
        <h1 className="font-semibold">{groupInfo?.name ?? "undefined"}</h1>
        <h2 className="text-sm">
          Chat active until{" "}
          {groupInfo?.end_date &&
            new Date(groupInfo.end_date).toLocaleDateString([], {
              month: "short",
              day: "numeric",
            }) +
              " • " +
              new Date(groupInfo.end_date).toLocaleTimeString([], {
                hour: "2-digit",
                minute: "2-digit",
              })}
        </h2>
        <Ping connected={connected} />
      </div>
      <Chat
        groupId={groupInfo?.group_id ?? -1}
        user={user}
        updatePing={(isConnected) => setConnected(isConnected)}
        tempGroupKey={groupInfo?.chat_key}
        password={password}
        isTempChat={true}
      />
    </div>
  );
}
