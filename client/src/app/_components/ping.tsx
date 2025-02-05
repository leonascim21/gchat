interface Props {
  connected: boolean;
}

export default function Ping({ connected }: Props) {
  return (
    <div className="flex">
      {connected ? (
        <>
          <span className="absolute inline-flex h-3 w-3 rounded-full bg-green-400 opacity-40 animate-ping motion-safe:duration-[3s]"></span>
          <span className="relative inline-flex h-3 w-3 rounded-full bg-green-500"></span>
        </>
      ) : (
        <>
          <span className="absolute inline-flex h-3 w-3 rounded-full bg-red-400 opacity-40 animate-ping"></span>
          <span className="relative inline-flex h-3 w-3 rounded-full bg-red-500"></span>
        </>
      )}
    </div>
  );
}
