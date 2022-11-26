#/bin/bash

cd rust/ && docker-compose up -d
cd ../
cd flask/ && docker-compose up -d
cd ../
cd ror/ && docker compose up -d
cd ../
cd goapi/ && docker compose up -d
cd ../
cd java/ && docker compose up -d
cd ../
cd nodejs/ && docker compose up -d
cd ../



echo "All in localhost:"

echo "JAVA http://localhost:8080"
echo "NODE http://localhost:7000"
echo "PYTHON http://localhost:5000"
echo "GO http://localhost:9000"
echo "RUBY ON RAILS  http://localhost:3000"
echo "RUST  http://localhost:2000"