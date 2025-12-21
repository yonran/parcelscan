#!/bin/bash

set -eu -o pipefail


usage() {
  cat <<EOF
Usage:
    download_socrata.sh --id <id> [OPTIONS]

Options:
    -h --help Show this screen
    --if-modified-since=<timestamp>
              Only download if the rowsUpdatedAt metadata exceeds timestamp (unix seconds)
EOF
}

if_modified_since=0
id=

while [ $# -ne 0 ]; do
  case "$1" in
    --id) shift; id="$1" ;;
    --id=*) id="${1#*=}" ;;
    --if-modified-since) shift; if_modified_since="$1" ;;
    --if-modified-since=*) if_modified_since="${1#*=}" ;;
    -h|help) usage; exit 0;;
    --*) echo >&2 "unknown arg $1"; usage >&2; exit 1;;
    *) echo >&2 "unexpected param $1"; usage >&2; exit 1;;
  esac
  shift
done

if [ -z "$id" ]; then
  echo >&2 "Missing required option --id"
  exit 1;
fi


if [ -z "$SOCRATA_API_KEY_ID" ] || [ -z "$SOCRATA_API_KEY_SECRET" ]; then
  echo >&2 "Missing Socrata API credentials; skipping."
  exit 1
fi

mkdir -p data
curl --no-progress-meter --fail --user "$SOCRATA_API_KEY_ID:$SOCRATA_API_KEY_SECRET" \
  "https://data.sfgov.org/api/views/$id.json" \
  --output "data/$id-meta.json"

rows_updated_at=$(jq -r '.rowsUpdatedAt // empty' "data/$id-meta.json")
if [ -z "$rows_updated_at" ]; then
  echo >&2 "Error: rowsUpdatedAt not found in metadata."
  head -c 2000 "data/$id-meta.json"
  exit 1
fi

if [ "$rows_updated_at" -le "$if_modified_since" ]; then
  echo >&2 "No update needed; rowsUpdatedAt=$rows_updated_at <= --if-modified-since=$if_modified_since."
  echo "did_update=false"
  exit 0
fi


curl --no-progress-meter --fail --user "$SOCRATA_API_KEY_ID:$SOCRATA_API_KEY_SECRET" \
  "https://data.sfgov.org/api/v3/views/$id/export.csv" \
  --output "data/$id.csv"

echo "did_update=true"
echo "rows_updated_at=$rows_updated_at"

