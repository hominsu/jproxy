target "metadata" {}

group "default" {
  targets = [
    "jproxy",
  ]
}

target "cross" {
  platforms = [
    "linux/arm64",
    "linux/amd64"
  ]
}

target "jproxy" {
  inherits = ["metadata", "cross"]
  cache-from = ["type=gha"]
  cache-to = ["type=gha,mode=max"]
  context    = "."
  dockerfile = "deploy/Dockerfile"
}
