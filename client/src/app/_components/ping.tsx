interface Props {
  connected: boolean;
}

export default function Ping({ connected }: Props) {
  return (
    <div
      className={`flex items-center gap-2 text-sm ${
        connected ? "text-green-500" : "text-red-500"
      }`}
    >
      <div
        className={`w-2 h-2 rounded-full animate-pulse ${
          connected ? "bg-green-500" : "bg-red-500"
        }`}
      ></div>
      {connected ? "Connected" : "Disconnected"}
    </div>
  );
}
