'use client';

import { useEffect, useState, useRef } from 'react';

export default function Home() {
  const [messages, setMessages] = useState<string[]>([]);
  const [inputMessage, setInputMessage] = useState('');
  const [connected, setConnected] = useState(false);
  const wsRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    const ws = new WebSocket('ws://localhost:3012/socket');
    wsRef.current = ws;

    ws.onopen = () => {
      setConnected(true);
      console.log('Connected to WebSocket server');
    };

    ws.onmessage = (event) => {
      setMessages(prev => [...prev, event.data]);
    };

    ws.onclose = () => {
      setConnected(false);
      console.log('Disconnected from WebSocket server');
    };

    return () => {
      ws.close();
    };
  }, []);

  const sendMessage = (e: React.FormEvent) => {
    e.preventDefault();
    if (inputMessage.trim() && wsRef.current) {
      wsRef.current.send(inputMessage);
      setInputMessage('');
    }
  };

  return (
    <div className="min-h-screen bg-gray-100 p-4">
      <div className="max-w-4xl mx-auto bg-white rounded-lg shadow-lg overflow-hidden">
        <div className="p-4 bg-blue-600 text-white">
          <h1 className="text-xl font-bold">WebSocket Chat</h1>
          <p className="text-sm">
            Status: {connected ? 'Connected' : 'Disconnected'}
          </p>
        </div>

        <div className="h-[60vh] overflow-y-auto p-4 space-y-2">
          {messages.map((message, index) => (
            <div key={index} className="bg-gray-100 p-2 rounded-lg">
              {message}
            </div>
          ))}
        </div>

        <form onSubmit={sendMessage} className="p-4 border-t">
          <div className="flex gap-2">
            <input
              type="text"
              value={inputMessage}
              onChange={(e) => setInputMessage(e.target.value)}
              placeholder="Type your message..."
              className="flex-1 px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              disabled={!connected}
            />
            <button
              type="submit"
              className="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50"
              disabled={!connected}
            >
              Send
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
