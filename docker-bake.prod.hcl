target "frontend" {
  secret = [
    {
      type = "env"
      id = "GRAPHQL_GITHUB_API_TOKEN"
    }
  ]
}

