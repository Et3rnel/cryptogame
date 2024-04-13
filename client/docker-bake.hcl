variable "TAG" {
  default = "latest"
}

group "default" {
  targets = ["client-development", "client-production"]
}

target "client-development" {
  dockerfile = "Dockerfile.dev"
  context = "."
  tags = ["svelte-client:dev-${TAG}"]
  args = {
    BUILD_ENV = "development"
  }
}

target "client-production" {
  dockerfile = "Dockerfile"
  context = "."
  tags = ["svelte-client:prod-${TAG}"]
  args = {
    BUILD_ENV = "production"
  }
}
