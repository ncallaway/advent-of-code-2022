#!/bin/bash

if [[ -z $1 ]] ; then echo 'no day given'; exit 1; fi
if [[ -z $2 ]] ; then echo 'no puzzle given'; exit 1; fi

APP_ROOT="$(dirname "$(dirname "$(readlink -fm "$0")")")"

DAY="day-$1"
PUZZLE="puzzle-$2"

cd $APP_ROOT

mkdir -p solutions
cd solutions

if [[ -d "$PUZZLE" ]] ; then echo "puzzle $DAY/$PUZZLE already exists"; exit 1; fi

echo "preparing puzzle for $DAY, $PUZZLE"
PROJECT="$DAY-$PUZZLE"
cargo new $PROJECT
mkdir -p "$PROJECT/input"
touch $PROJECT/input/sample
touch $PROJECT/input/puzzle
