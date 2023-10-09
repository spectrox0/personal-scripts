#!/bin/bash

REVIEWER="osgmendez"  # Cambia esto por el nombre de usuario del revisor en GitHub.
MERGE_AFTER_CREATING=false

# Si se pasa el flag --merge, cambia el valor de MERGE_AFTER_CREATING a true.
for arg in "$@"; do
    if [ "$arg" == "--merge" ]; then
        MERGE_AFTER_CREATING=true
    fi
done

# Obtén la rama actual en la que te encuentras.
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

# Verifica si estás en 'dev'. No quieres crear un PR desde 'dev' a 'dev'.
if [ "$CURRENT_BRANCH" == "dev" ]; then
    echo "Ya estás en la rama 'dev'. Cambia a otra rama para crear un PR hacia 'dev'."
    exit 1
fi

# Crea un PR desde la rama actual hacia 'dev' y agrega el revisor.
PR_URL=$(gh pr create --base dev --head $CURRENT_BRANCH --reviewer $REVIEWER --fill)

echo "Pull request creado desde $CURRENT_BRANCH hacia dev con éxito!"

# Si se ha establecido MERGE_AFTER_CREATING a true, se mergea el PR.
if $MERGE_AFTER_CREATING; then
    PR_NUMBER=$(echo $PR_URL | grep -oE "[0-9]+$")
    gh pr merge $PR_NUMBER --merge
    echo "Pull request #$PR_NUMBER ha sido mergeado exitosamente!"
fi
