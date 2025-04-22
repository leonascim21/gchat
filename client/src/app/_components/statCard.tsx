interface Props {
  title: string;
  stat: string;
}

export default function StatCard({ title, stat }: Props) {
  return (
    <div className="flex flex-col items-center justify-center rounded-md border border-gray-200 dark:border-gray-700 p-4 text-center">
      <h1 className="text-sm font-semibold text-center">{title}</h1>
      <p className="text-primary text-xl">{stat}</p>
    </div>
  );
}
