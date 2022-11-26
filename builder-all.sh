docker build -t dude/man:v2 .


#/bin/bash

cd rust/ && docker build -t apitest/rust .
cd ../
cd flask/ && docker build -t apitest/python .
cd ../
cd ror/ && docker build -t apitest/ror .
cd ../
cd goapi/ && docker build -t apitest/go .
cd ../
cd java/ && docker build -t apitest/java .
cd ../
cd nodejs/ && docker build -t apitest/node .
cd ../
