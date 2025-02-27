#!/bin/sh
ssh -v -i /tmp/remote-login-key.pem -o StrictHostKeyChecking=no -o UserKnownHostsFile=/tmp/known_hosts "$SERVER_USER@$SERVER_DNS_OR_IP" << EOF
  mkdir -p ~/.docker && echo "$DOCKER_AUTH_CONFIG" > ~/.docker/config.json
  docker pull "$CI_REGISTRY_IMAGE:$CI_COMMIT_SHORT_SHA"
  docker stop $CONTAINER_NAME || true  # Stop any running instance
  docker rm $CONTAINER_NAME || true    # Remove stopped instance
  docker run -d --name $CONTAINER_NAME -p 80:8080 "$CI_REGISTRY_IMAGE:$CI_COMMIT_SHORT_SHA"
EOF
