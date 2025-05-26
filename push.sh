docker build -t vecstore .
docker tag vecstore-extractor:latest 355673447504.dkr.ecr.eu-central-1.amazonaws.com/vecstore-extractor:latest
docker push 355673447504.dkr.ecr.eu-central-1.amazonaws.com/vecstore-extractor:latest
