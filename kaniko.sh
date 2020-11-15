#!/bin/sh -e

echo "{\"auths\":{\"$CI_REGISTRY\":{\"username\":\"$CI_REGISTRY_USER\",\"password\":\"$CI_REGISTRY_PASSWORD\"}}}" > /kaniko/.docker/config.json
echo "Working around https://github.com/GoogleContainerTools/kaniko/issues/595"
rm -f .dockerignore
/kaniko/executor --context $CI_PROJECT_DIR \
  --dockerfile $CI_PROJECT_DIR/$DOCKERFILE \
  --build-arg SFOS_VERSION="$SFOS_VERSION" \
  --build-arg SFOS_ARCH="$SFOS_ARCH" \
  --build-arg CROSS_NAME_SHORT="$CROSS_NAME_SHORT" \
  --build-arg CROSS_NAME_RUST="$CROSS_NAME_RUST" \
  --destination $CI_REGISTRY_IMAGE/platform-$SFOS_ARCH-$SFOS_VERSION:$CI_COMMIT_REF_SLUG \
  --cache=true \

