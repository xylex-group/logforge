export type LogLevel = 'ERROR' | 'WARN' | 'INFO' | 'DEBUG';

export interface LogEvent {
  ts:         string;
  level:      LogLevel;
  service:    string;
  host?:      string;
  env?:       string;
  trace_id?:  string;
  span_id?:   string;
  msg:        string;
  path?:      string;
  method?:    string;
  status?:    number;
  latency_ms?: number;
  user_id?:   string;
  slow:       boolean;
  sampled:    boolean;
}

export interface LogsResponse {
  count:  number;
  events: LogEvent[];
}
