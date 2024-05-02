#!/bin/sh

DB_IMAGE="postgres:16"

## DB settings with their default values, if no custom value is provided as an environment variable.
DB_USER="${POSTGRES_USER:=test}"
DB_PASSWORD="${POSTGRES_PASSWORD:=test}"
DB_NAME="${POSTGRES_DB:=test}"
DB_PORT="${POSTGRES_PORT:=5443}"
DB_HOST="${POSTGRES_HOST:=localhost}"
CONTAINER_NAME="test"
docker run \
      -e POSTGRES_USER=${DB_USER} \
      -e POSTGRES_PASSWORD=${DB_PASSWORD} \
      -e POSTGRES_DB=${DB_NAME} \
      -p "${DB_PORT}":5432 \
      -d \
      --name ${CONTAINER_NAME} \
      ${DB_IMAGE} \
      -N 500
      # ^ Increased the maximum number of connections for testing purposes.

# Keep polling Postgres for its readiness.
until PGPASSWORD="${DB_PASSWORD}" psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "⏳ Postgres is still unavailable. Trying again in 1 sec ..."
  sleep 1
done

echo "✅ Postgres is up and accessible on host ${DB_HOST} and port ${DB_PORT}."
