import type { LogLevel } from '@/lib/types';

const styles: Record<LogLevel, string> = {
  ERROR: 'bg-red-900/60 text-red-300 border border-red-700/50',
  WARN:  'bg-amber-900/60 text-amber-300 border border-amber-700/50',
  INFO:  'bg-blue-900/60 text-blue-300 border border-blue-700/50',
  DEBUG: 'bg-gray-800 text-gray-400 border border-gray-700/50',
};

export function LevelBadge({ level }: { level: LogLevel }) {
  return (
    <span className={`inline-block text-[10px] font-mono font-semibold px-1.5 py-0.5 rounded ${styles[level]}`}>
      {level}
    </span>
  );
}
