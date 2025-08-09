#! /bin/bash

query=`cat ./queries/github.graphql`
query="$(echo $query | sed -e 's/\"/\\\"/g')"

curl -X POST https://api.github.com/graphql \
  -H "Authorization: Bearer $GITHUB_TOKEN" \
  -H 'Content-Type: application/json' \
  -d "{\"query\": \"$query\"}"
