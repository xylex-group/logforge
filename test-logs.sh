#!/usr/bin/env bash
# test-logs.sh — fire a burst of test events at the logforge ingest endpoint
# Usage: ./test-logs.sh [host]   (default: localhost:3001)

HOST="${1:-localhost:3001}"
URL="http://$HOST/api/ingest"
SERVICES=("api-gateway" "auth-service" "order-service" "payment-service")
PATHS=("/api/orders" "/api/auth/login" "/api/products" "/health")
METHODS=("GET" "POST" "DELETE")
NOW=$(date -u +"%Y-%m-%dT%H:%M:%S.000Z")

send() {
  curl -s -o /dev/null -X POST "$URL" \
    -H "Content-Type: application/json" \
    -d "$1"
}

echo "Sending test events to $URL ..."

# 1. Normal INFO traffic
for i in $(seq 1 20); do
  SVC=${SERVICES[$((RANDOM % ${#SERVICES[@]}))]}
  PATH=${PATHS[$((RANDOM % ${#PATHS[@]}))]}
  LATENCY=$((RANDOM % 300 + 20))
  send "{\"ts\":\"$NOW\",\"level\":\"INFO\",\"service\":\"$SVC\",\"msg\":\"request handled\",\"path\":\"$PATH\",\"method\":\"GET\",\"status\":200,\"latency_ms\":$LATENCY,\"trace_id\":\"$(uuidgen | tr -d '-' | head -c 16)\"}"
done

# 2. A cluster of errors (tests burst suppression)
echo "Sending error burst..."
for i in $(seq 1 15); do
  send "{\"ts\":\"$NOW\",\"level\":\"ERROR\",\"service\":\"payment-service\",\"msg\":\"upstream timeout after 5003ms\",\"path\":\"/api/payments\",\"status\":504,\"latency_ms\":5003,\"trace_id\":\"deadbeef$(printf '%08x' $RANDOM)\"}"
done

# 3. Some slow requests
for i in $(seq 1 5); do
  SVC=${SERVICES[$((RANDOM % ${#SERVICES[@]}))]}
  LATENCY=$((RANDOM % 3000 + 1200))
  send "{\"ts\":\"$NOW\",\"level\":\"WARN\",\"service\":\"$SVC\",\"msg\":\"slow database query detected\",\"latency_ms\":$LATENCY,\"trace_id\":\"slowtrace$(printf '%08x' $i)\"}"
done

# 4. Auth failures
for i in $(seq 1 3); do
  send "{\"ts\":\"$NOW\",\"level\":\"WARN\",\"service\":\"auth-service\",\"msg\":\"authentication failed\",\"path\":\"/api/auth/login\",\"status\":401,\"user_id\":\"usr_unknown_$i\"}"
done

echo "Done. Open http://localhost:3000 to see results."
