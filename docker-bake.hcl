variable "CF_SITEKEY" {
  default = "$CF_SITEKEY"

  validation {
    condition = CF_SITEKEY != ""
    error_message = "CF_SITEKEY environment variable is empty"
  }
}

group "default" {
  targets = [ "frontend", "backend" ]
}

target "frontend" {
  tags = [ "biixie/biixie.com:frontend-latest" ]
  content = "."
  dockerfile = "frontend/Dockerfile"

  args = {
    CF_SITEKEY = CF_SITEKEY
  }
}

target "backend" {
  tags = [ "biixie/biixie.com:backend-latest" ]
  contenet = "."
  dockerfile = "backend/Dockerfile"
}
