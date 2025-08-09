# Get zola binary
FROM ghcr.io/getzola/zola:v0.20.0 AS zola-base

COPY ./frontend /project


# Workaround for lack of /bin/sh in Zola's base image.
FROM debian:bookworm-slim AS zola

COPY --from=zola-base /bin/zola /bin/zola
COPY --from=zola-base /project /project

WORKDIR /project

RUN apt-get -y update;
RUN apt-get -y install curl

RUN cat ./queries/github.graphql

# Query GitHub's GraphQL API for my pinned projects.
# I do it here because it is easier than trying to do it in Zola directly.
RUN --mount=type=secret,id=GRAPHQL_GITHUB_API_TOKEN,env=GITHUB_TOKEN \
  sh ./query_projects.sh > ./projects.json

RUN cat ./projects.json

# Build website
ARG CF_SITEKEY # Used to inject the Turnstile sitekey into the site.
ENV CF_SITEKEY=$CF_SITEKEY
RUN zola build

# Serve the built website.
FROM caddy:2.9-alpine

WORKDIR /
COPY --from=zola /project/caddy/Caddyfile /etc/caddy/Caddyfile
COPY --from=zola /project/public /srv
