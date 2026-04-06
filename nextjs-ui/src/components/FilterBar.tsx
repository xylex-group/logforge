'use client';

interface FilterBarProps {
  level:      string;
  service:    string;
  search:     string;
  onLevel:    (v: string) => void;
  onService:  (v: string) => void;
  onSearch:   (v: string) => void;
  onRefresh:  () => void;
  autoRefresh: boolean;
  onAutoRefresh: (v: boolean) => void;
}

const LEVELS = ['', 'ERROR', 'WARN', 'INFO', 'DEBUG'];

export function FilterBar({
  level, service, search,
  onLevel, onService, onSearch,
  onRefresh, autoRefresh, onAutoRefresh,
}: FilterBarProps) {
  return (
    <div className="flex flex-wrap gap-2 px-4 py-3 border-b border-gray-800 bg-gray-950/80 backdrop-blur sticky top-0 z-10">
      {/* Level filter */}
      <select
        value={level}
        onChange={(e) => onLevel(e.target.value)}
        className="bg-gray-900 border border-gray-700 text-gray-300 text-xs rounded px-2 py-1.5 focus:outline-none focus:border-gray-500"
      >
        <option value="">All levels</option>
        {LEVELS.slice(1).map((l) => (
          <option key={l} value={l}>{l}</option>
        ))}
      </select>

      {/* Service filter */}
      <input
        type="text"
        placeholder="Service…"
        value={service}
        onChange={(e) => onService(e.target.value)}
        className="bg-gray-900 border border-gray-700 text-gray-300 text-xs rounded px-2 py-1.5 w-36 focus:outline-none focus:border-gray-500 placeholder-gray-600"
      />

      {/* Full-text search */}
      <input
        type="text"
        placeholder="Search messages…"
        value={search}
        onChange={(e) => onSearch(e.target.value)}
        className="bg-gray-900 border border-gray-700 text-gray-300 text-xs rounded px-2 py-1.5 flex-1 min-w-[160px] focus:outline-none focus:border-gray-500 placeholder-gray-600"
      />

      <div className="flex items-center gap-2 ml-auto">
        {/* Auto-refresh toggle */}
        <label className="flex items-center gap-1.5 text-xs text-gray-500 cursor-pointer">
          <input
            type="checkbox"
            checked={autoRefresh}
            onChange={(e) => onAutoRefresh(e.target.checked)}
            className="accent-purple-500"
          />
          Live
        </label>

        <button
          onClick={onRefresh}
          className="text-xs text-gray-400 border border-gray-700 rounded px-3 py-1.5 hover:bg-gray-800 transition-colors"
        >
          Refresh
        </button>
      </div>
    </div>
  );
}
