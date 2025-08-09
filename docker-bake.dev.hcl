target "frontend" {
  secret = [
    {
      type = "file"
      id = "GRAPHQL_GITHUB_API_TOKEN"
      src = "./frontend/secrets/github_token"
    }
  ]
}

