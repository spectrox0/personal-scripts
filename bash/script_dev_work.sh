# Sample repo
DEFAULT_REPO="opratel-core/platform-omanagerv2-front"
# Sample branch
DEFAULT_BRANCH="dev"
REPO=$DEFAULT_REPO
BRANCH=$DEFAULT_BRANCH

while [[ $# -gt 0 ]]; do
    key="$1"
    case $key in
        --repo)
        REPO="$2"
        shift # pasa al siguiente argumento
        shift # pasa al valor del argumento anterior
        ;;
        --branch)
        BRANCH="$2"
        shift # pasa al siguiente argumento
        shift # pasa al valor del argumento anterior
        ;;
        *)
        shift # pasa al siguiente argumento
        ;;
    esac
done

LAST_TAG=$(gh release list --repo $REPO --limit 1 | awk '{print $1}')

echo "The last version tag --> $LAST_TAG"

if [ -z "$LAST_TAG" ]; then
    echo "No se encontró un tag anterior. Terminando el script."
    exit 1
fi

LAST_TAG=${LAST_TAG#v}

MAJOR=$(echo $LAST_TAG | awk -F. '{print $1}')
MINOR=$(echo $LAST_TAG | awk -F. '{print $2}')
PATCH=$(echo $LAST_TAG | awk -F. '{print $3}')

PATCH=$((PATCH + 1))

NEW_TAG="v${MAJOR}.${MINOR}.${PATCH}"

gh release create $NEW_TAG -t $NEW_TAG --notes "Release $NEW_TAG" --repo $REPO --target $BRANCH

echo "New tag $NEW_TAG created for branch $BRANCH!"