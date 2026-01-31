aws ecr get-login-password --profile kencho --region eu-central-1 | docker login --username AWS --password-stdin 355673447504.dkr.ecr.eu-central-1.amazonaws.com

docker buildx build --platform linux/amd64 \
  -t 355673447504.dkr.ecr.eu-central-1.amazonaws.com/vecstore-back:latest \
  --push .
