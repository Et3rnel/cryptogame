variable "TAG" {
  default = "latest"
}

group "default" {
  targets = ["development", "production"]
}

target "development" {
  dockerfile = "Dockerfile.dev"
  context = "."
  tags = ["cryptogame:dev-${TAG}"]
  args = {
    BUILD_ENV = "development"
  }
}

target "production" {
  dockerfile = "Dockerfile"
  context = "."
  tags = ["cryptogame:prod-${TAG}"]
  args = {
    BUILD_ENV = "production"
  }
}
