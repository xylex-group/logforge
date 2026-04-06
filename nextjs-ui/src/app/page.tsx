'use client';

import { useState, useEffect, useMemo } from 'react';
import useSWR from 'swr';
import { fetcher, buildLogsUrl } from '@/lib/api';
import type { LogEvent, LogsResponse } from '@/lib/types';
import { StatCard }  from '@/components/StatCard';
import { FilterBar } from '@/components/FilterBar';
import { LogRow }    from '@/components/LogRow';

export default function DashboardPage() {
  const [level,       setLevel]       = useState('');
  const [service,     setService]     = useState('');
  const [search,      setSearch]      = useState('');
  const [autoRefresh, setAutoRefresh] = useState(true);
  const [tick,        setTick]        = useState(0);

  // Auto-refresh every 3 seconds when enabled
  useEffect(() => {
    if (!autoRefresh) return;
    const id = setInterval(() => setTick((t) => t + 1), 3000);
    return () => clearInterval(id);
  }, [autoRefresh]);

  const url = buildLogsUrl({ level, service, search, limit: 300 });

  const { data, mutate, isLoading } = useSWR<LogsResponse>(
    [url, tick],
    ([u]) => fetcher(u as string),
    { revalidateOnFocus: false }
  );

  const events: LogEvent[] = data?.events ?? [];

  // Derived stats
  const stats = useMemo(() => {
    const errors  = events.filter((e) => e.level === 'ERROR').length;
    const warns   = events.filter((e) => e.level === 'WARN').length;
    const slow    = events.filter((e) => e.slow).length;
    const latencies = events
      .map((e) => e.latency_ms)
      .filter((v): v is number => v != null)
      .sort((a, b) => a - b);
    const p99 = latencies.length
      ? latencies[Math.floor(latencies.length * 0.99)]
      : null;
    return { errors, warns, slow, p99, total: events.length };
  }, [events]);

  return (
    <div className="min-h-screen bg-gray-950 flex flex-col">
      {/* Header */}
      <header className="border-b border-gray-800 px-4 py-3 flex items-center gap-3">
        <div className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse" />
        <span className="text-sm font-semibold text-gray-200 tracking-tight">LogForge</span>
        <span className="text-xs text-gray-600">infrastructure log viewer</span>
        {isLoading && (
          <span className="ml-auto text-[10px] text-gray-600 animate-pulse">loading…</span>
        )}
      </header>

      {/* Stats row */}
      <div className="grid grid-cols-2 sm:grid-cols-4 gap-3 px-4 py-4 border-b border-gray-800">
        <StatCard label="Events"   value={stats.total.toLocaleString()}  />
        <StatCard label="Errors"   value={stats.errors}  color={stats.errors  > 0 ? 'red'   : 'default'} />
        <StatCard label="Warnings" value={stats.warns}   color={stats.warns   > 0 ? 'amber' : 'default'} />
        <StatCard
          label="p99 latency"
          value={stats.p99 != null ? `${stats.p99}ms` : '—'}
          sub={stats.slow > 0 ? `${stats.slow} slow requests` : undefined}
          color={stats.p99 != null && stats.p99 > 1000 ? 'amber' : 'default'}
        />
      </div>

      {/* Filters */}
      <FilterBar
        level={level}           onLevel={setLevel}
        service={service}       onService={setService}
        search={search}         onSearch={setSearch}
        autoRefresh={autoRefresh} onAutoRefresh={setAutoRefresh}
        onRefresh={() => mutate()}
      />

      {/* Log stream */}
      <main className="flex-1 overflow-auto">
        {events.length === 0 && !isLoading ? (
          <div className="flex flex-col items-center justify-center h-64 text-gray-700">
            <p className="text-sm">No events match the current filters.</p>
            <p className="text-xs mt-1">
              Send logs via <code className="font-mono text-gray-600">POST /api/ingest</code>
            </p>
          </div>
        ) : (
          <div>
            {events.map((e, i) => (
              <LogRow key={`${e.ts}-${i}`} event={e} />
            ))}
          </div>
        )}
      </main>

      {/* Footer */}
      <footer className="border-t border-gray-800 px-4 py-2 flex items-center gap-4 text-[10px] text-gray-700">
        <span>{stats.total.toLocaleString()} events loaded</span>
        <span>·</span>
        <span>Rust sampler active — burst suppression + trace-aware + probabilistic</span>
        <span className="ml-auto">{autoRefresh ? 'live · 3s' : 'paused'}</span>
      </footer>
    </div>
  );
}
