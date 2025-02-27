#!/bin/sh
ssh -v -i /tmp/remote-login-key.pem -o StrictHostKeyChecking=no -o UserKnownHostsFile=/tmp/known_hosts "$SERVER_USER@$SERVER_DNS_OR_IP" << EOF

  docker pull "$CI_REGISTRY_IMAGE:$CI_COMMIT_SHORT_SHA"
  docker stop "$CI_REGISTRY_IMAGE-container" || true  # Stop any running instance
  docker rm "$CI_REGISTRY_IMAGE-container" || true    # Remove stopped instance
  docker run -d --name "$CI_REGISTRY_IMAGE-container" -p $API_PORT:$API_PORT --network writeonce-network "$CI_REGISTRY_IMAGE:$CI_COMMIT_SHORT_SHA"
EOF
