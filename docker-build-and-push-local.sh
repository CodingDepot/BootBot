#!/bin/bash

# BEFORE starting this script, call 'docker login' to authenticate.
# Replace 'namespace' and 'project' as needed

namespace="codingdepot"
project="boot-bot"
version=$(cargo metadata --no-deps | jq -r .packages[0].version)

docker build --network=host -t ${namespace}/${project}:${version} .
docker push ${namespace}/${project}:${version}
