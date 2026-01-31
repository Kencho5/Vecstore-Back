docker buildx build --platform linux/amd64 -t vecstore-back .
docker tag vecstore-back:latest 355673447504.dkr.ecr.eu-central-1.amazonaws.com/vecstore-back:latest
docker push 355673447504.dkr.ecr.eu-central-1.amazonaws.com/vecstore-back:latest
