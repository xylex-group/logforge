interface StatCardProps {
  label: string;
  value: string | number;
  sub?:  string;
  color?: 'default' | 'red' | 'amber' | 'green' | 'blue';
}

const colorMap = {
  default: 'text-gray-100',
  red:     'text-red-400',
  amber:   'text-amber-400',
  green:   'text-emerald-400',
  blue:    'text-blue-400',
};

export function StatCard({ label, value, sub, color = 'default' }: StatCardProps) {
  return (
    <div className="bg-gray-900 border border-gray-800 rounded-lg px-4 py-3">
      <p className="text-[11px] text-gray-500 uppercase tracking-wider mb-1">{label}</p>
      <p className={`text-2xl font-semibold tabular-nums ${colorMap[color]}`}>{value}</p>
      {sub && <p className="text-[11px] text-gray-600 mt-0.5">{sub}</p>}
    </div>
  );
}
