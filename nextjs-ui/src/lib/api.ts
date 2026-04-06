import type { LogsResponse } from './types';

export const fetcher = (url: string) =>
  fetch(url).then((r) => {
    if (!r.ok) throw new Error(`fetch error ${r.status}`);
    return r.json();
  });

export function buildLogsUrl(params: {
  level?:   string;
  service?: string;
  search?:  string;
  limit?:   number;
}) {
  const q = new URLSearchParams();
  if (params.level)   q.set('level',   params.level);
  if (params.service) q.set('service', params.service);
  if (params.search)  q.set('search',  params.search);
  q.set('limit', String(params.limit ?? 200));
  return `/api/logs?${q.toString()}`;
}

export async function ingestLog(event: Omit<import('./types').LogEvent, 'slow' | 'sampled'>) {
  return fetch('/api/ingest', {
    method:  'POST',
    headers: { 'Content-Type': 'application/json' },
    body:    JSON.stringify(event),
  });
}
