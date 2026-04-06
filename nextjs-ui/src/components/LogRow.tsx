'use client';
import { useState } from 'react';
import { formatDistanceToNow } from 'date-fns';
import type { LogEvent } from '@/lib/types';
import { LevelBadge } from './LevelBadge';

export function LogRow({ event }: { event: LogEvent }) {
  const [expanded, setExpanded] = useState(false);
  const ts = new Date(event.ts);

  return (
    <div
      className={`border-b border-gray-800/60 cursor-pointer hover:bg-gray-900/60 transition-colors ${
        event.level === 'ERROR' ? 'border-l-2 border-l-red-600' :
        event.level === 'WARN'  ? 'border-l-2 border-l-amber-500' :
        'border-l-2 border-l-transparent'
      }`}
      onClick={() => setExpanded((v) => !v)}
    >
      {/* Summary row */}
      <div className="flex items-start gap-3 px-4 py-2.5 font-mono text-[12px]">
        <span className="text-gray-600 whitespace-nowrap pt-0.5 min-w-[90px]">
          {formatDistanceToNow(ts, { addSuffix: true })}
        </span>
        <LevelBadge level={event.level} />
        <span className="text-purple-400 whitespace-nowrap">{event.service}</span>
        {event.status && (
          <span className={
            event.status >= 500 ? 'text-red-400' :
            event.status >= 400 ? 'text-amber-400' :
            'text-gray-500'
          }>{event.status}</span>
        )}
        {event.latency_ms != null && (
          <span className={event.slow ? 'text-amber-400' : 'text-gray-600'}>
            {event.latency_ms}ms{event.slow ? ' ⚠' : ''}
          </span>
        )}
        <span className="text-gray-300 flex-1 truncate">{event.msg}</span>
        {event.trace_id && (
          <span className="text-gray-700 text-[10px] hidden xl:block">
            {event.trace_id.slice(0, 8)}
          </span>
        )}
      </div>

      {/* Expanded detail */}
      {expanded && (
        <div className="px-4 pb-3 pt-1">
          <pre className="text-[11px] text-gray-400 bg-gray-950 border border-gray-800 rounded p-3 overflow-x-auto leading-relaxed">
            {JSON.stringify(event, null, 2)}
          </pre>
        </div>
      )}
    </div>
  );
}
