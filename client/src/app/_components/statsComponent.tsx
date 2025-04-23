"use client";
import { useEffect, useState } from "react";
import StatCard from "./statCard";
import axios from "axios";

export default function StatsComponent() {
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);
  const [longestMessage, setLongestMessage] = useState("");
  const [messagesSent, setMessagesSent] = useState(0);
  const [favoriteGroup, setFavoriteGroup] = useState("");
  const [bestFriend, setBestFriend] = useState("");

  useEffect(() => {
    const token = localStorage.getItem("token");

    if (!token) {
      return;
    }

    axios
      .get(`https://api.gchat.cloud/user/get-user-stats?token=${token}`)
      .then((response) => {
        setLongestMessage(response.data.longest_message);
        setMessagesSent(response.data.messages_sent);
        setFavoriteGroup(response.data.favorite_group);
        setBestFriend(response.data.best_friend);
      })
      .catch(() => setIsError(true))
      .finally(() => setIsLoading(false));
  }, []);

  if (isLoading)
    return (
      <div className="flex justify-center items-center">
        <div className="w-12 h-12 border-4 border-primary border-t-transparent rounded-full animate-spin"></div>
      </div>
    );

  if (isError)
    return (
      <div className="flex justify-center items-center">An Error Occured</div>
    );

  return (
    <div className="grid grid-cols-2 gap-4">
      <StatCard title="Messages Sent" stat={String(messagesSent)} />
      <StatCard title="Longest Message Sent" stat={longestMessage} />
      <StatCard title="Favorite Group" stat={favoriteGroup} />
      <StatCard title="Best Friend" stat={bestFriend} />
    </div>
  );
}
