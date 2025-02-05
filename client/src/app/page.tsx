"use client";

import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Send } from "lucide-react";
import { useEffect, useState, useRef } from "react";
import Ping from "./_components/ping";

export default function Home() {
  const [messages, setMessages] = useState<string[]>([]);
  const [inputMessage, setInputMessage] = useState("");
  const [connected, setConnected] = useState(false);
  const wsRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    const ws = new WebSocket("wss://ws.gchat.cloud");
    wsRef.current = ws;

    ws.onopen = () => {
      setConnected(true);
      console.log("Connected to WebSocket server");
    };

    ws.onmessage = (event) => {
      setMessages((prev) => [...prev, event.data]);
    };

    ws.onclose = () => {
      setConnected(false);
      console.log("Disconnected from WebSocket server");
    };

    return () => {
      ws.close();
    };
  }, []);

  const sendMessage = (e: React.FormEvent) => {
    e.preventDefault();
    if (inputMessage.trim() && wsRef.current) {
      wsRef.current.send(inputMessage);
      setInputMessage("");
    }
  };

  return (
    <div className="flex flex-col justify-center items-center h-screen bg-slate-600 md:p-4">
      <Card className="flex flex-col bg-slate-200 border-0 shadow-xl sm:w-[100vw] sm:h-[100vh] md:w-[70vw] md:h-[80vh] ">
        <CardHeader className="mb-auto border-b border-slate-300">
          <h1 className="text-xl font-bold">WebSocket Chat</h1>
          <div className="flex flex-row gap-3 items-center">
            <p className="text-sm">
              Status: {connected ? "Connected" : "Disconnected"}
            </p>
            <Ping connected={connected} />
          </div>
        </CardHeader>
        <CardContent className="flex-grow overflow-y-auto p-4">
          <div className="flex flex-col gap-2 w-full">
            {messages.map((message, index) =>
              index % 2 == 0 ? (
                <div key={index} className="flex justify-end">
                  <Card className="bg-slate-800 border-0 shadow-lg py-2 px-4 text-white w-fit rounded-full">
                    {message}
                  </Card>
                </div>
              ) : (
                <div key={index} className="flex justify-start">
                  <Card className="bg-slate-300 border-0 shadow-lg py-2 px-4 text-black w-fit rounded-full">
                    {message}
                  </Card>
                </div>
              )
            )}
          </div>
        </CardContent>
        <CardFooter className="mt-auto border-t border-slate-300">
          <form
            onSubmit={sendMessage}
            className="flex flex-row gap-2 mt-3 w-full justify-center"
          >
            <Input
              className="bg-slate-100 border-slate-300 w-[60%]"
              type="text"
              value={inputMessage}
              onChange={(e) => setInputMessage(e.target.value)}
              placeholder="Type your message..."
              disabled={!connected}
            />
            <Button className="w-[5%]" disabled={!inputMessage}>
              <Send />
            </Button>
          </form>
        </CardFooter>
      </Card>
    </div>
  );
}
