#!/bin/bash

# BEFORE starting this script, call 'docker login' to authenticate.
# Replace 'namespace' and 'project' as needed

namespace="codingdepot"
project="boot-bot"

version=$(cargo metadata --no-deps | jq -r .packages[0].version)
id=$(docker build --network=host -t ${namespace}/${project} .)

docker tag ${namespace}/${project} ${namespace}/${project}:${version}-pi
docker tag ${namespace}/${project} ${namespace}/${project}:latest-pi

docker push ${namespace}/${project}:${version}-pi
docker push ${namespace}/${project}:latest-pi
